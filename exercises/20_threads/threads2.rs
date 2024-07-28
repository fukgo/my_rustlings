// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::{Arc,Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            //更新一个被多个线程共享的值之前，你需要采取一些行动来确保线程安全,例如使用 Mutex 或 RwLock
            // TODO: You must take an action before you update a shared value.
            let mut write_status = status_shared.lock().unwrap();
            write_status.jobs_done +=1;

        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    let read_status = status.lock().unwrap();

    println!("Jobs done: {}", read_status.jobs_done);
}
