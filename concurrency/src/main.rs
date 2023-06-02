use rand::Rng;

#[test]
fn shared_counter() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time;

    let number_of_threads = 10;

    // Spawn a few threads to increment a shared variable (non-atomically)
    // Here we're using an Arc to share memory among threads, and the data inside
    // the Arc is protected with a mutex.
    let data = Arc::new(Mutex::new(0));

    (0..number_of_threads)
        .map(|thread_index| {
            let data = data.clone();

            thread::spawn(move || {
                thread::sleep(time::Duration::from_millis(1000));
                println!("Hi from thread {thread_index}");
                // The shared state can only be accessed once the lock is held.
                // Our non-atomic increment is safe because we're the only thread
                // which can access the shared state when the lock is held.
                //
                // We unwrap() the return value to assert that we are not expecting
                // threads to ever fail while holding the lock.
                let mut data = data.lock().unwrap();
                *data += 1;
                // the lock is unlocked here when `data` goes out of scope.
            })
        })
        .collect::<Vec<_>>() // we collect it here so it's not lazy
        .into_iter()
        .for_each(|h| h.join().unwrap());

    let final_value = *data.lock().unwrap();
    println!("N: {final_value}")
}

// For the channel fans:
// use std::sync::mpsc::channel;
// let (tx, rx) = channel();
// tx.send(()).unwrap();
// rx.recv().unwrap();

fn main() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time;

    const philosophers: usize = 3;

    let forks = [0; philosophers].map(|_| Arc::new(Mutex::new(())));

    (0..philosophers)
        .map(|thread_index| {
            let left: Arc<Mutex<()>> = forks[thread_index].clone();
            let right = forks[(thread_index + 1) % philosophers].clone();

            thread::spawn(move || loop {
                thread::sleep(time::Duration::from_secs(
                    rand::thread_rng().gen_range(0..5),
                ));

                println!("Philosopher {thread_index} waiting to pick up left");
                let _left = left.lock().unwrap();

                thread::sleep(time::Duration::from_secs_f64(rand::random::<f64>()));

                println!("Philosopher {thread_index} waiting to pick up right");
                let _right = right.lock().unwrap();

                println!("Philosopher {thread_index} dropping");
            })
        })
        .collect::<Vec<_>>() // we collect it here so it's not lazy
        .into_iter()
        .for_each(|h| h.join().unwrap());
}
