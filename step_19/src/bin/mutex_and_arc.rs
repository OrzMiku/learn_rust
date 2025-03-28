use std::sync::{Mutex, Arc};
use std::thread;

struct Database {
    connections: Vec<u32>
}

impl Database {
    fn new() -> Database {
        Database { connections: vec![] }
    }

    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

fn main() {
    // Arc is reference-counting smart pointer which is thread-safe.
    // Mutex is a mutual exclusion lock that prevents data races.
    let db = Arc::new(Mutex::new(Database::new()));
    
    let mut thread_handles = vec![];

    for id in 0..10 {
        // Arc::clone only increments ref count, does not create a new instance.
        let db_clone = Arc::clone(&db);

        let thread_handle = thread::spawn(move || {
            // Get the lock
            let mut db_lock = db_clone.lock().unwrap();
            db_lock.connect(id);
            // The lock will be released when variable is dropped.
            // We can use drop function to drop lock manually.
        });

        thread_handles.push(thread_handle);
    }

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }

    let db_lock = db.lock().unwrap();
    for connection in &db_lock.connections {
        println!("connection {}", connection);
    }
}