use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

#[tokio::main]
async fn main() {
    println!("Starting sleep");
    SleepFuture::new(Duration::from_millis(1000)).await;
    println!("Finished sleeping");
}

/// 自定义的异步休眠 Future
/// 功能类似于 `tokio::time::sleep`，但用标准库的线程实现
struct SleepFuture {
    /// 共享状态，用 `Mutex` 保护，确保线程安全
    shared: Arc<Mutex<SharedState>>,
}

/// 共享状态，存储 Waker 和当前状态
struct SharedState {
    /// 存储 `Waker`，用于在休眠完成后唤醒任务
    waker: Option<Waker>,
    /// 当前 Future 的状态
    state: SleepState,
}

/// Future 的状态机
#[derive(PartialEq)]
enum SleepState {
    /// 初始状态，尚未开始休眠
    Init,
    /// 正在休眠（后台线程已启动）
    Sleeping,
    /// 休眠完成，可以返回 `Poll::Ready`
    Done,
}

impl SleepFuture {
    /// 创建一个新的 `SleepFuture`，并启动后台线程计时
    fn new(duration: Duration) -> SleepFuture {
        // 初始化共享状态
        let shared = Arc::new(Mutex::new(SharedState {
            waker: None,
            state: SleepState::Init,
        }));

        // 克隆 `Arc`，用于在线程中修改状态
        let shared_clone = Arc::clone(&shared);

        // 启动后台线程，模拟异步休眠
        thread::spawn(move || {
            // 阻塞当前线程，模拟休眠
            thread::sleep(duration);

            // 休眠完成后，更新状态并唤醒任务
            let mut shared = shared_clone.lock().unwrap();
            shared.state = SleepState::Done; // 标记为完成

            // 如果已经存储了 `Waker`，则唤醒任务
            if let Some(waker) = shared.waker.take() {
                waker.wake();
            }
        });

        SleepFuture { shared }
    }
}

impl Future for SleepFuture {
    type Output = ();

    /// 实现 `Future` 的 `poll` 方法
    /// 如果休眠完成，返回 `Poll::Ready(())`，否则返回 `Poll::Pending`
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared = self.shared.lock().unwrap();

        match shared.state {
            SleepState::Init => {
                // 第一次调用 `poll`，存储 `Waker` 以便后续唤醒
                shared.waker = Some(cx.waker().clone());
                shared.state = SleepState::Sleeping; // 进入休眠状态
                Poll::Pending
            }
            SleepState::Sleeping => {
                // 仍在休眠，继续等待
                Poll::Pending
            }
            SleepState::Done => {
                // 休眠完成，返回 `Ready`
                Poll::Ready(())
            }
        }
    }
}
