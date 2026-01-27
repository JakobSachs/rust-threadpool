use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

struct Task {
    pub func: Box<dyn FnOnce() -> () + Send>,
}

pub struct Pool {
    threads: Vec<JoinHandle<()>>,
    queue: Arc<Mutex<VecDeque<Task>>>,
    done: Arc<AtomicBool>,
}

impl Pool {
    pub fn new(size: usize) -> Pool {
        let mut threads = Vec::with_capacity(size);
        let queue = Arc::new(Mutex::new(VecDeque::<Task>::new()));
        let done = Arc::new(AtomicBool::new(false));

        // spawn threads
        for _ in 0..size {
            let queue: Arc<Mutex<VecDeque<Task>>> = queue.clone();
            let done = done.clone();
            threads.push(thread::spawn(move || {
                let mut sleep_counter = 1;
                loop {
                    // check if we should stop
                    if done.load(Ordering::Relaxed) {
                        break;
                    }

                    // get task
                    let task = {
                        let mut queue = queue.lock().unwrap();
                        queue.pop_front()
                    };

                    if let Some(task) = task {
                        (task.func)();
                    } else {
                        // no task, sleep a bit with exponential backoff
                        thread::sleep(std::time::Duration::from_nanos(1 << sleep_counter));
                        sleep_counter += 1;
                    }
                }
            }));
        }

        Pool {
            threads: threads,
            queue: queue,
            done: done,
        }
    }

    //publishes a new task to the pool
    pub fn submit<F: FnOnce() -> () + Send + 'static>(&self, func: F) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(Task {
            func: Box::new(func),
        });
    }

    // waits for all tasks to finish, and then joins all threads
    pub fn join_all(self) {
        // wait for all tasks to finish
        let mut sleep_counter = 1;
        while !self.queue.lock().unwrap().is_empty() {
            thread::sleep(std::time::Duration::from_nanos(1 << sleep_counter));
            sleep_counter += 1;
        }

        self.done.store(true, Ordering::Relaxed);
        for thread in self.threads.into_iter() {
            thread.join().unwrap();
        }
    }
}
