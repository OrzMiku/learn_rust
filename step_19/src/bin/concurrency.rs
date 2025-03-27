use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    // We can use `thread::spawn()` to spawn a thread
    // `spawn()` returns a `JoinHandle`
    let handle = thread::spawn(|| {
        for num in 0..20 {
            // `sleep` puts the thread to sleep; in this example, it's used to make observation easier
            sleep(Duration::from_millis(10));
            println!("child thread: {}", num);
        }
    });

    for num in 0..10 {
        sleep(Duration::from_millis(10));
        println!("main thread: {}", num);
    }

    // When the main thread reaches this point, if `join()` is not called,
    // the program may exit before the child thread completes
    // `join()` blocks the main thread until the child thread finishes executing
    handle.join().unwrap();
}
