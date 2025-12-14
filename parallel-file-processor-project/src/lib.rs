pub mod threadpool;
pub mod analysis;

use threadpool::ThreadPool;
use analysis::{FileAnalysis}; 
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::path::{Path, PathBuf};
use std::fs;
use std::thread;
use std::time::Duration;

// 3.1: Define Global State for Results, Progress, and Cancellation
struct ProcessorContext {
    
    results: Mutex<Vec<FileAnalysis>>,
    
    files_completed: AtomicUsize, 
    
    total_files: AtomicUsize, 
    
    cancellation_token: Arc<std::sync::atomic::AtomicBool>, 
}


pub fn run() {
    // --- Configuration ---
    let thread_count = 16; // Adjust based on your CPU cores
    let target_dir = Path::new("./gutenberg_books"); 

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
    
    
    let file_paths: Vec<PathBuf> = match fs::read_dir(target_dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file()) 
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
        
        
        let job = move || {
            
            if context_clone.cancellation_token.load(Ordering::SeqCst) { 
                return;
            }

            let analysis_result = analysis::analyze_file(&path, &context_clone.cancellation_token);
            
            // 1. Store the results
            context_clone.results.lock().unwrap().push(analysis_result);
            
            // 2. Update the progress counter
            context_clone.files_completed.fetch_add(1, Ordering::SeqCst);
        };
        
        pool.execute(job);
    } 

    // --- 3.4: Progress Tracking ---
    let total = context.total_files.load(Ordering::SeqCst);
    while context.files_completed.load(Ordering::SeqCst) < total {
        let completed = context.files_completed.load(Ordering::SeqCst);
        println!("Progress: {} / {} files completed...", completed, total);
        thread::sleep(Duration::from_secs(1)); 
        
    }
    
    
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
