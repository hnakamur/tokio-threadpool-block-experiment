use tokio_threadpool::{ThreadPool, blocking};

use futures::Future;
use futures::future::{lazy, poll_fn};

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    // This is a *blocking* channel
    let (tx, rx) = mpsc::channel();

    // Spawn a thread to send a message
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send("hello").unwrap();
    });

    let pool = ThreadPool::new();

    pool.spawn(lazy(move || {
        // Because `blocking` returns `Poll`, it is intended to be used
        // from the context of a `Future` implementation. Since we don't
        // have a complicated requirement, we can use `poll_fn` in this
        // case.
        poll_fn(move || {
            blocking(|| {
                let msg = rx.recv().unwrap();
                println!("message = {}", msg);
            }).map_err(|_| panic!("the threadpool shut down"))
        })
    }));

    // Wait for the task we just spawned to complete.
    pool.shutdown_now().wait().unwrap();
    thread::sleep(Duration::from_millis(1000));
}
