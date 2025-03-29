use std::time::Duration;
use tokio::time::sleep;

#[tokio::main(flavor="current_thread")] // single thread and time slice
// #[tokio::main] // thread poll
async fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        // Spawn a new task for each iteration
        // The `tokio::spawn` function is used to create a new asynchronous task
        // that runs concurrently with the rest of the program.
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    // Spawn a CPU-intensive task
    let cpu_handle = tokio::spawn(async {
        // Cpu intensive task will block the thread if run in the same thread
        // and will not yield to other tasks until it is done.
        // cpu_intensive_task().await;

        // We can run CPU intensive task in a separate thread pool
        let _res = tokio::task::spawn_blocking(|| {
            cpu_intensive_task();
        }).await;
    });
    handles.push(cpu_handle);

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function(i: i32) {
    println!("[{}] I'm a async function!", i);
    let s1 = read_from_database().await;
    println!("[{}] First result: {}", i, s1);
    let s2 = read_from_database().await;
    println!("[{}] Second result: {}", i, s2);
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(500)).await;
    format!("DB Result")
}

// async fn cpu_intensive_task() {
fn cpu_intensive_task() {
    let mut sum = 0;
    for _ in 0..1000000000 {
        sum += 1;
    }
    println!("Sum: {}", sum);
}