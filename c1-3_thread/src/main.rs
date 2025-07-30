use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    {
        let start = Instant::now();
        let _ = fibonacci(50);
        let duration = start.elapsed();
        // fibonacci(50) in 82.945175416s
        println!("fibonacci(50) in {duration:?}");
    }
    {
        let start = Instant::now();
        let mut handles = vec![];
        for _ in 0..4 {
            let handle = thread::spawn(|| fibonacci(50));
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
        let duration = start.elapsed();
        // 4 threads fibonacci(50) took 82.860646375s
        println!("4 threads fibonacci(50) took {duration:?}");
    }
    {
        let shared_data = Arc::new((Mutex::new(false), Condvar::new()));
        let shared_data_clone = Arc::clone(&shared_data);
        let STOP = Arc::new(AtomicBool::new(false));
        let STOP_CLONE = Arc::clone(&STOP);

        let _background_thread = thread::spawn(move || {
            let (lock, cvar) = &*shared_data_clone;
            let mut received_value = lock.lock().unwrap();
            while !STOP.load(Relaxed) {
                received_value = cvar.wait(received_value).unwrap();
                println!("Received value: {}", *received_value);
            }
        });

        let updater_thread = thread::spawn(move || {
            let (lock, cvar) = &*shared_data;
            let values = [false, true, false, true];

            for i in 0..4 {
                let update_value = values[i as usize];
                println!("Updating value to {update_value}...");
                *lock.lock().unwrap() = update_value;
                cvar.notify_one();
                thread::sleep(Duration::from_secs(4));
            }
            STOP_CLONE.store(true, Relaxed);
            println!("STOP has been updated");
            cvar.notify_one();
        });
        updater_thread.join().unwrap();
    }
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
