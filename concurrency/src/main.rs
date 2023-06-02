fn main() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    const N: usize = 100;

    // Spawn a few threads to increment a shared variable (non-atomically)
    // Here we're using an Arc to share memory among threads, and the data inside
    // the Arc is protected with a mutex.
    let data = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();
    for _ in 0..N {
        let data = data.clone();
        let handle = thread::spawn(move || {
            // The shared state can only be accessed once the lock is held.
            // Our non-atomic increment is safe because we're the only thread
            // which can access the shared state when the lock is held.
            //
            // We unwrap() the return value to assert that we are not expecting
            // threads to ever fail while holding the lock.
            let mut data = data.lock().unwrap();
            *data += 1;
            // the lock is unlocked here when `data` goes out of scope.
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    let final_value = *data.lock().unwrap();
    println!("N: {final_value}")
}

// For the channel fans:
// use std::sync::mpsc::channel;
// let (tx, rx) = channel();
// tx.send(()).unwrap();
// rx.recv().unwrap();
