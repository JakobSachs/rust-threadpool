use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

use itertools::Itertools;

struct Task {
    pub func: Box<dyn FnOnce() -> () + Send>,
}

pub struct Pool {
    threads: Vec<JoinHandle<()>>,
    condvar: Arc<Condvar>,
    executing: Arc<AtomicU32>,
    queue: Arc<Mutex<Vec<Task>>>,
    done: Arc<AtomicBool>,
}

impl Pool {
    pub fn new(size: usize) -> Pool {
        let mut threads = Vec::with_capacity(size);
        let queue = Arc::new(Mutex::new(Vec::<Task>::new()));
        let done = Arc::new(AtomicBool::new(false));
        let executing = Arc::new(AtomicU32::new(0));
        let condvar = Arc::new(Condvar::new());

        // spawn threads
        for _ in 0..size {
            let queue: Arc<Mutex<Vec<Task>>> = queue.clone();
            let done = done.clone();
            let condvar = condvar.clone();
            let executing = executing.clone();

            // worker thread code
            threads.push(thread::spawn(move || {
                loop {
                    let mut guard = queue.lock().unwrap();
                    while guard.is_empty() && !done.load(Ordering::Acquire) {
                        guard = condvar.wait(guard).unwrap();
                    }
                    if guard.is_empty() {
                        break; // all tasks are done and done is set
                    }
                    let task = guard.pop().unwrap();
                    drop(guard);
                    executing.fetch_add(1, Ordering::Relaxed);
                    (task.func)();
                    executing.fetch_sub(1, Ordering::Relaxed);
                }
            }));
        }

        Pool {
            threads: threads,
            queue: queue,
            condvar: condvar,
            executing: executing,
            done: done,
        }
    }

    //publishes a new task to the pool
    pub fn submit<F: FnOnce() -> () + Send + 'static>(&self, func: F) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(Task {
            func: Box::new(func),
        });
        self.condvar.notify_one();
    }

    // publishes a new task per item in the iterator
    pub fn submit_iter<F: Fn(T) + Send + Sync + 'static, T: Send + 'static>(
        &self,
        func: Arc<F>,
        iter: impl IntoIterator<Item = T>,
        chunk_size: usize,
    ) {
        let mut queue = self.queue.lock().unwrap();
        for chunk in &iter.into_iter().chunks(chunk_size) {
            let func = Arc::clone(&func);
            let batch: Vec<T> = chunk.collect();
            queue.push(Task {
                func: Box::new(move || {
                    for item in batch {
                        func(item);
                    }
                }),
            });
        }
        self.condvar.notify_all();
    }

    // waits for all tasks to finish, and then joins all threads
    pub fn join_all(self) {
        // wait for all tasks to finish
        let mut sleep_counter = 1;
        self.done.store(true, Ordering::Release);
        self.condvar.notify_all();
        while self.executing.load(Ordering::Relaxed) > 0 {
            std::thread::sleep(std::time::Duration::from_nanos(std::cmp::max(
                1000,
                std::cmp::min(100_000_000, 1 << sleep_counter),
            )));
            sleep_counter += 1;
        }
        for thread in self.threads.into_iter() {
            thread.join().unwrap();
        }
    }
}
