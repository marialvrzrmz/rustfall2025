// src/threadpool.rs

use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};

// 1.1: Define Job type
// Type alias for a Job: a trait object that is Send and has a static lifetime
type Job = Box<dyn FnOnce() + Send + 'static>;

// Shared state between the ThreadPool handle and the workers
struct SharedState {
    job_queue: Mutex<Vec<Job>>,
    notifier: Condvar,
    // Used to signal workers to gracefully shut down
    terminate: AtomicBool, 
}

// The main interface for the user
pub struct ThreadPool {
    workers: Vec<Worker>,
    // 1.2: Shared state wrapped in Arc
    state: Arc<SharedState>,
}

impl ThreadPool {
    // 1.3: Implement ThreadPool::new
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let state = Arc::new(SharedState {
            job_queue: Mutex::new(Vec::new()),
            notifier: Condvar::new(),
            terminate: AtomicBool::new(false),
        });

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Clone Arc components for each worker
            workers.push(Worker::new(id, Arc::clone(&state)));
        }

        ThreadPool { workers, state }
    }

    // 1.4: Implement ThreadPool::execute
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // 1. Lock the queue
        let mut queue = self.state.job_queue.lock().unwrap();
        
        // 2. Push the new job
        queue.push(job);
        
        // Mutex guard drops here, and then we notify one worker
        // 3. Signal ONE waiting thread
        self.state.notifier.notify_one();
    }
}


// 1.6: Implement Graceful Shutdown (Drop)
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down thread pool...");
        
        // 1. Set the termination signal
        self.state.terminate.store(true, Ordering::SeqCst);
        
        // 2. Wake up all workers so they can check the termination signal
        self.state.notifier.notify_all();

        // 3. Wait for all worker threads to finish
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                // Ignore errors if the thread has already panicked
                let _ = thread.join(); 
            }
        }
        println!("Thread pool shut down successfully.");
    }
}


// Structure to manage an individual worker thread
struct Worker {
    id: usize,
    // Option is used so we can 'take' the JoinHandle during shutdown
    thread: Option<JoinHandle<()>>, 
}

impl Worker {
    fn new(id: usize, state: Arc<SharedState>) -> Worker {
        let thread = thread::spawn(move || {
            // 1.5: Implement the Worker Loop
            loop {
                let job = {
                    let mut queue = state.job_queue.lock().unwrap();

                    // 1. Wait *only* if the queue is empty AND we are not terminating.
                    // This ensures if jobs exist, we proceed directly to pop.
                    while queue.is_empty() && !state.terminate.load(Ordering::SeqCst) {
                        queue = state.notifier.wait(queue).unwrap();
                    }
                    
                    // 2. CRITICAL CHECK: After waking up, if the queue is *still* empty, 
                    // AND the termination signal is SET, we exit.
                    if queue.is_empty() && state.terminate.load(Ordering::SeqCst) {
                        break; 
                    }
                    
                    // 3. Pop the job. If we reached here, either a job was present, 
                    // or we are terminating, but we must check for the job first.
                    queue.pop()
                }; // Mutex lock implicitly released here

                if let Some(job) = job {
                    job(); 
                }
            }
            println!("Worker {} finished.", id);
        });

        Worker { id, thread: Some(thread) }
    }
}


// src/threadpool.rs (Add this block at the end)

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

    /// Test that a thread pool can execute a simple task and increment a shared counter.
    #[test]
    fn test_simple_execution() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));
        
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }
        
        // Wait long enough for all tasks to complete.
        thread::sleep(Duration::from_millis(500)); 
        
        assert_eq!(counter.load(Ordering::SeqCst), 10, "Not all 10 jobs were executed.");
    }

    /// Test that the thread pool shuts down gracefully (Drop implementation).
    #[test]
    fn test_graceful_shutdown() {
        let size = 4;
        let pool = ThreadPool::new(size);
        let finished_counter = Arc::new(AtomicUsize::new(0));

        // Submit tasks that take a short time
        for _ in 0..size * 2 {
            let counter_clone = Arc::clone(&finished_counter);
            pool.execute(move || {
                thread::sleep(Duration::from_millis(50));
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }
        
        // When 'pool' goes out of scope, Drop is called, shutting down workers.
        drop(pool);
        
        // Assert that all submitted tasks were completed before shutdown.
        assert_eq!(finished_counter.load(Ordering::SeqCst), size * 2, 
                   "ThreadPool did not wait for all jobs to complete during shutdown.");
    }
}