use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    //Using a mutex in a multi-threaded context,
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            //The call to lock would fail and return an error if another thread holding the lock panicked.
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        //The call to join will make sure that all the threads have finished their work before continuing.
        handle.join().unwrap();
    }

    println("Result: {}", *counter.lock().unwrap());
}