use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

use itertools::Itertools;

struct Task {
    pub func: Box<dyn FnOnce() -> () + Send>,
}

pub struct Pool {
    threads: Vec<JoinHandle<()>>,
    work_condvar: Arc<Condvar>,
    completion_condvar: Arc<Condvar>,
    executing: Arc<AtomicU32>,
    queue: Arc<Mutex<VecDeque<Task>>>,
    done: Arc<AtomicBool>,
}

impl Pool {
    pub fn new(size: usize) -> Pool {
        let mut threads = Vec::with_capacity(size);
        let queue = Arc::new(Mutex::new(VecDeque::<Task>::new()));
        let done = Arc::new(AtomicBool::new(false));
        let executing = Arc::new(AtomicU32::new(0));
        let work_condvar = Arc::new(Condvar::new());
        let completion_condvar = Arc::new(Condvar::new());

        // spawn threads
        for _ in 0..size {
            let queue: Arc<Mutex<VecDeque<Task>>> = queue.clone();
            let done = done.clone();
            let work_condvar = work_condvar.clone();
            let completion_condvar = completion_condvar.clone();
            let executing = executing.clone();

            // worker thread code
            threads.push(thread::spawn(move || {
                loop {
                    let mut guard = queue.lock().unwrap();
                    while guard.is_empty() && !done.load(Ordering::Acquire) {
                        guard = work_condvar.wait(guard).unwrap();
                    }
                    if guard.is_empty() {
                        break; // all tasks are done and done is set
                    }
                    let task = guard.pop_front().unwrap();
                    drop(guard);
                    executing.fetch_add(1, Ordering::Relaxed);
                    (task.func)();
                    let prev = executing.fetch_sub(1, Ordering::Release);
                    // If we might be the last executing task, notify completion
                    if prev == 1 {
                        completion_condvar.notify_one();
                    }
                }
            }))
        }

        Pool {
            threads: threads,
            queue: queue,
            work_condvar: work_condvar,
            completion_condvar: completion_condvar,
            executing: executing,
            done: done,
        }
    }

    //publishes a new task to the pool
    pub fn submit<F: FnOnce() -> () + Send + 'static>(&self, func: F) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(Task {
            func: Box::new(func),
        });
        self.work_condvar.notify_one();
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
            // prealloc batch with chunk_size capacity
            let mut batch = Vec::with_capacity(chunk_size);
            batch.extend(chunk);
            queue.push_back(Task {
                func: Box::new(move || {
                    for item in batch {
                        func(item);
                    }
                }),
            });
        }
        self.work_condvar.notify_all();
    }

    // waits for all tasks to finish, and then joins all threads
    pub fn join_all(self) {
        // Signal shutdown
        self.done.store(true, Ordering::Release);
        self.work_condvar.notify_all();

        // Wait for all work to complete (queue empty + no executing tasks)
        let mut guard = self.queue.lock().unwrap();
        while !guard.is_empty() || self.executing.load(Ordering::Acquire) > 0 {
            guard = self.completion_condvar.wait(guard).unwrap();
        }
        drop(guard);

        // Join all threads
        for thread in self.threads.into_iter() {
            thread.join().unwrap();
        }
    }
}
