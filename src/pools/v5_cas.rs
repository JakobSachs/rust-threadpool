use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

use lock_free::Queue;

use itertools::Itertools;

struct Task {
    pub func: Box<dyn FnOnce() -> () + Send>,
}

pub struct Pool {
    threads: Vec<JoinHandle<()>>,
    completion_condvar: Arc<Condvar>,
    completion_mutex: Arc<Mutex<()>>,
    executing: Arc<AtomicU32>,
    queue: Arc<Queue<Task>>,
    done: Arc<AtomicBool>,
}

impl Pool {
    pub fn new(size: usize) -> Pool {
        let mut threads = Vec::with_capacity(size);
        let queue = Arc::new(Queue::<Task>::new(1 << 16));
        let done = Arc::new(AtomicBool::new(false));
        let executing = Arc::new(AtomicU32::new(0));
        let completion_condvar = Arc::new(Condvar::new());
        let completion_mutex = Arc::new(Mutex::new(()));

        // spawn threads
        for _ in 0..size {
            let queue = queue.clone();
            let done = done.clone();
            let completion_condvar = completion_condvar.clone();
            let completion_mutex = completion_mutex.clone();
            let executing = executing.clone();

            // worker thread code
            threads.push(thread::spawn(move || {
                loop {
                    if let Some(task) = queue.dequeue() {
                        executing.fetch_add(1, Ordering::Relaxed);
                        (task.func)();
                        let prev = executing.fetch_sub(1, Ordering::Release);
                        // If we might be the last executing task, notify completion
                        if prev == 1 {
                            let _lock = completion_mutex.lock().unwrap();
                            completion_condvar.notify_one();
                        }
                    } else {
                        // No work available - check if we should exit
                        if done.load(Ordering::Acquire) {
                            break;
                        }
                        // Spin briefly before checking again
                        std::hint::spin_loop();
                    }
                }
            }))
        }

        Pool {
            threads: threads,
            queue: queue,
            completion_condvar: completion_condvar,
            completion_mutex: completion_mutex,
            executing: executing,
            done: done,
        }
    }

    //publishes a new task to the pool
    pub fn submit<F: FnOnce() -> () + Send + 'static>(&self, func: F) {
        self.queue.enqueue(Task {
            func: Box::new(func),
        });
    }

    // publishes a new task per item in the iterator
    pub fn submit_iter<F: Fn(T) + Send + Sync + 'static, T: Send + 'static>(
        &self,
        func: Arc<F>,
        iter: impl IntoIterator<Item = T>,
        chunk_size: usize,
    ) {
        for chunk in &iter.into_iter().chunks(chunk_size) {
            let func = Arc::clone(&func);
            // prealloc batch with chunk_size capacity
            let mut batch = Vec::with_capacity(chunk_size);
            batch.extend(chunk);
            self.queue.enqueue(Task {
                func: Box::new(move || {
                    for item in batch {
                        func(item);
                    }
                }),
            });
        }
    }

    // waits for all tasks to finish, and then joins all threads
    pub fn join_all(self) {
        // Signal shutdown
        self.done.store(true, Ordering::Release);

        // Wait for all work to complete (queue empty + no executing tasks)
        let mut guard = self.completion_mutex.lock().unwrap();
        while !self.queue.is_empty() || self.executing.load(Ordering::Acquire) > 0 {
            guard = self.completion_condvar.wait(guard).unwrap();
        }
        drop(guard);

        // Join all threads
        for thread in self.threads.into_iter() {
            thread.join().unwrap();
        }
    }
}
