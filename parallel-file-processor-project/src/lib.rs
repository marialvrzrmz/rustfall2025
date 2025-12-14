pub mod threadpool;
pub mod analysis;

use threadpool::ThreadPool;
use analysis::{FileAnalysis}; 
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::{Path, PathBuf};
use std::fs;
use std::thread; // <-- FIX 5
use std::time::Duration;

// 3.1: Define Global State for Results, Progress, and Cancellation
struct ProcessorContext {
    // The final collection of all results
    results: Mutex<Vec<FileAnalysis>>,
    // Simple counter for real-time progress updates
    files_completed: AtomicUsize, 
    // Total number of files found to process
    total_files: AtomicUsize, 
    // Cancellation signal (required for concurrency without external libraries)
    cancellation_token: Arc<std::sync::atomic::AtomicBool>, 
}


pub fn run() {
    // --- Configuration ---
    let thread_count = 8; // Adjust based on your CPU cores
    let target_dir = Path::new("./gutenberg_books"); // *** Make sure this directory exists ***

    println!("Starting Parallel File Processor with {} threads...", thread_count);

    // --- 3.2: Initialize Thread Pool and Context ---
    let pool = ThreadPool::new(thread_count);
    
    let context = Arc::new(ProcessorContext {
        results: Mutex::new(Vec::new()),
        files_completed: AtomicUsize::new(0),
        total_files: AtomicUsize::new(0),
        cancellation_token: Arc::new(std::sync::atomic::AtomicBool::new(false)),
    });

    // --- 3.2: Find Files and Dispatch Tasks ---
    
    // We need to collect all paths first to get the total count for the progress tracker
    let file_paths: Vec<PathBuf> = match fs::read_dir(target_dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file()) // Only process files, skip subdirectories for now
            .collect(),
        Err(e) => {
            eprintln!("Error reading directory {}: {}", target_dir.display(), e);
            return;
        }
    };

    context.total_files.store(file_paths.len(), Ordering::SeqCst);
    println!("Found {} files to process.", file_paths.len());

    // 3.3: Submit Tasks
    for path in file_paths {
        let context_clone = Arc::clone(&context);
        
        // The job closure: runs the analysis and updates the shared context
        let job = move || {
            // Check for cancellation before starting the job
            if context_clone.cancellation_token.load(Ordering::SeqCst) {
                // Log cancellation error if needed, but for now, just exit.
                return;
            }

            let analysis_result = analysis::analyze_file(&path, &context_clone.cancellation_token);
            
            // 1. Store the results
            context_clone.results.lock().unwrap().push(analysis_result);
            
            // 2. Update the progress counter
            context_clone.files_completed.fetch_add(1, Ordering::SeqCst);
        };
        
        pool.execute(job);
    } // Pool starts processing immediately

    // --- 3.4: Progress Tracking (Simplified) ---
    // Wait for all tasks to complete by waiting for the counter to match the total.
    let total = context.total_files.load(Ordering::SeqCst);
    while context.files_completed.load(Ordering::SeqCst) < total {
        let completed = context.files_completed.load(Ordering::SeqCst);
        println!("Progress: {} / {} files completed...", completed, total);
        thread::sleep(Duration::from_secs(1)); 
        // Example: If a user signal (like 'q' on terminal) occurs, 
        // you would set context.cancellation_token.store(true, Ordering::SeqCst); 
    }
    
    // Ensure all threads finish gracefully (ThreadPool::drop is called here)
    drop(pool); 

    println!("\n--- Processing Complete ---");
    
    // --- Final Output and Error Reporting ---
    let final_results = context.results.lock().unwrap();
    let total_errors = final_results.iter().flat_map(|a| &a.errors).count();
    let total_time: Duration = final_results.iter().map(|a| a.processing_time).sum();

    println!("Total files processed: {}", final_results.len());
    println!("Total errors encountered: {}", total_errors);
    println!("Total combined processing time: {:?}", total_time);
    
    println!("\n--- Processing Complete ---");

    // --- Final Results Detail ---
    let final_results = context.results.lock().unwrap();
    
    println!("\n--- Detailed Analysis Summary ---");
    for analysis in final_results.iter() {
        let error_msg = if analysis.errors.is_empty() {
            " (OK)"
        } else {
            " (ERRORS)"
        };
        
        // This line uses filename and stats/word_count, eliminating the unused field warnings!
        println!(
            "File: {} | Words: {} | Time: {:.2?} {}",
            analysis.filename,
            analysis.stats.word_count,
            analysis.processing_time,
            error_msg
        );
    }
    println!("---------------------------------");
    
}

// Helper function to show how file system traversal would work recursively (Bonus Feature)
// fn traverse_and_collect_files(dir: &Path, paths: &mut Vec<PathBuf>) -> io::Result<()> { ... }