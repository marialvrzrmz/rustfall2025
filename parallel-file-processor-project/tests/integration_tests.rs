use parallel_file_processor_project::threadpool::ThreadPool;
use parallel_file_processor_project::analysis::{FileAnalysis, ProcessingError};

// FIX E0433: The analysis function is now correctly referred to via the crate path
use parallel_file_processor_project::analysis;

use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicBool, Ordering}};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::Duration;
use std::io;

// tests/integration_tests.rs (continued)

// Helper struct mirroring the one in main.rs for testing
struct TestContext {
    results: Mutex<Vec<FileAnalysis>>,
    files_completed: AtomicUsize, 
    total_files: AtomicUsize, 
    cancellation_token: Arc<AtomicBool>, 
}


#[test]
fn test_error_and_success_handling() -> Result<(), io::Error> {
    // 1. SETUP: Create temporary files and error paths
    
    // Create a temporary directory for the test files
    let temp_dir = PathBuf::from("./test_temp_data");
    let _ = fs::create_dir_all(&temp_dir); // Ensure it exists

    // A. Success Case: A valid file
    let success_path = temp_dir.join("success.txt");
    fs::write(&success_path, "This is a test file.\nTwo lines, five words.")?;

    // B. Error Case 1: Non-existent file
    let nonexistent_path = PathBuf::from("./non_existent_file.txt");

    // C. Error Case 2: Attempt to read a directory as a file (IO Error)
    let directory_path = temp_dir.join("an_actual_directory");
    fs::create_dir(&directory_path)?; 

    let paths_to_process = vec![
        success_path, 
        nonexistent_path, 
        directory_path,
    ];
    let total_files = paths_to_process.len();
    
    // 2. INITIALIZE PROCESSOR
    let pool = ThreadPool::new(2); // Use a small pool
    let context = Arc::new(TestContext {
        results: Mutex::new(Vec::new()),
        files_completed: AtomicUsize::new(0),
        total_files: AtomicUsize::new(total_files),
        cancellation_token: Arc::new(AtomicBool::new(false)),
    });

    // 3. DISPATCH TASKS
    for path in paths_to_process {
        let context_clone = Arc::clone(&context);
        
        pool.execute(move || {
            // Note: We need a way to call analyze_file that is accessible
            // You may need to create a helper test function or make analyze_file public
            let analysis_result = analysis::analyze_file(&path, &context_clone.cancellation_token);
            
            context_clone.results.lock().unwrap().push(analysis_result);
            context_clone.files_completed.fetch_add(1, Ordering::SeqCst);
        });
    }

    // 4. WAIT FOR COMPLETION
    let total = context.total_files.load(Ordering::SeqCst);
    let timeout = Duration::from_secs(5);
    let start_time = std::time::Instant::now();

    while context.files_completed.load(Ordering::SeqCst) < total {
        if start_time.elapsed() > timeout {
            panic!("Test timed out waiting for files to complete.");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    
    // Ensure cleanup happens gracefully
    drop(pool); 

    // 5. ASSERTIONS: Verify success and error paths
    let final_results = context.results.lock().unwrap();

    // Assert 1: All tasks completed
    assert_eq!(final_results.len(), total_files, "Not all tasks were processed.");

    // Find the results for assertion
    let success_analysis = final_results.iter().find(|a| a.filename == "success.txt").unwrap();
    let nonexistent_analysis = final_results.iter().find(|a| a.filename == "non_existent_file.txt").unwrap();
    let directory_analysis = final_results.iter().find(|a| a.filename == "an_actual_directory").unwrap();

    // Assert 2: Success case stats are correct
    assert_eq!(success_analysis.stats.line_count, 2, "Success file analysis failed.");
    assert!(success_analysis.errors.is_empty(), "Success file unexpectedly had errors.");

    // Assert 3: Error case (Non-existent) was captured gracefully
    assert!(!nonexistent_analysis.errors.is_empty(), "Non-existent file failed to report an error.");
    assert!(matches!(nonexistent_analysis.errors[0], ProcessingError::IoError(_)), "Non-existent file reported wrong error type.");

    // Assert 4: Error case (Directory read) was captured gracefully
    assert!(!directory_analysis.errors.is_empty(), "Directory-read file failed to report an error.");
    assert!(matches!(directory_analysis.errors[0], ProcessingError::IoError(_)), "Directory-read file reported wrong error type.");

    // 6. CLEANUP
    fs::remove_dir_all(&temp_dir)?;
    
    Ok(())
}