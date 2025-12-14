

use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};


type Job = Box<dyn FnOnce() + Send + 'static>;


struct SharedState {
    job_queue: Mutex<Vec<Job>>,
    notifier: Condvar,
    
    terminate: AtomicBool, 
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    
    state: Arc<SharedState>,
}

impl ThreadPool {
    
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let state = Arc::new(SharedState {
            job_queue: Mutex::new(Vec::new()),
            notifier: Condvar::new(),
            terminate: AtomicBool::new(false),
        });

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            
            workers.push(Worker::new(id, Arc::clone(&state)));
        }

        ThreadPool { workers, state }
    }

    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        
        let mut queue = self.state.job_queue.lock().unwrap();
        
        
        queue.push(job);
        
        
        self.state.notifier.notify_one();
    }
}


// 1.6: Implement Graceful Shutdown (Drop)
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down thread pool...");
        
        
        self.state.terminate.store(true, Ordering::SeqCst);
        
        
        self.state.notifier.notify_all();

        
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                
                let _ = thread.join(); 
            }
        }
        println!("Thread pool shut down successfully.");
    }
}



struct Worker {
    id: usize,
    
    thread: Option<JoinHandle<()>>, 
}

impl Worker {
    fn new(id: usize, state: Arc<SharedState>) -> Worker {
        let thread = thread::spawn(move || {
            
            loop {
                let job = {
                    let mut queue = state.job_queue.lock().unwrap();

                    
                    while queue.is_empty() && !state.terminate.load(Ordering::SeqCst) {
                        queue = state.notifier.wait(queue).unwrap();
                    }
                    
                    
                    if queue.is_empty() && state.terminate.load(Ordering::SeqCst) {
                        break; 
                    }
                    
                    
                    queue.pop()
                }; 

                if let Some(job) = job {
                    job(); 
                }
            }
            println!("Worker {} finished.", id);
        });

        Worker { id, thread: Some(thread) }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

    
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
        
        
        thread::sleep(Duration::from_millis(500)); 
        
        assert_eq!(counter.load(Ordering::SeqCst), 10, "Not all 10 jobs were executed.");
    }

    
    #[test]
    fn test_graceful_shutdown() {
        let size = 4;
        let pool = ThreadPool::new(size);
        let finished_counter = Arc::new(AtomicUsize::new(0));

        
        for _ in 0..size * 2 {
            let counter_clone = Arc::clone(&finished_counter);
            pool.execute(move || {
                thread::sleep(Duration::from_millis(50));
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }
        
        
        drop(pool);
        
        
        assert_eq!(finished_counter.load(Ordering::SeqCst), size * 2, 
                   "ThreadPool did not wait for all jobs to complete during shutdown.");
    }
}