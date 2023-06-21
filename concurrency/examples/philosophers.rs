fn main() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time;

    const PHILOSOPHERS: usize = 3;

    let forks = [0; PHILOSOPHERS].map(|_| Arc::new(Mutex::new(())));

    (0..PHILOSOPHERS)
        .map(|thread_index| {
            let left: Arc<Mutex<()>> = forks[thread_index].clone();
            let right = forks[(thread_index + 1) % PHILOSOPHERS].clone();

            thread::spawn(move || loop {
                thread::sleep(time::Duration::from_secs_f64(rand::random::<f64>()));

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
