
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::fs; 
use std::path::Path;

// 2.1: Define the required output structs

#[derive(Debug)]
pub enum ProcessingError {
    IoError(String),
    EncodingError(String),
    
    Cancelled, 
}

#[derive(Debug)]
pub struct FileStats {
    pub word_count: usize,
    pub line_count: usize,
    pub char_frequencies: HashMap<char, usize>,
    pub size_bytes: u64,
}

#[derive(Debug)]
pub struct FileAnalysis {
    pub filename: String,
    pub stats: FileStats,
    pub errors: Vec<ProcessingError>,
    pub processing_time: Duration,
}


// 2.3: Implement Line and Word Count
fn analyze_text_counts(content: &str) -> (usize, usize) {
    let mut line_count = 0;
    let mut word_count = 0;

    for line in content.lines() {
        line_count += 1;
        
        word_count += line.split_whitespace().count();
    }

    (line_count, word_count)
}

// 2.4: Implement Character Frequency
fn analyze_char_frequencies(content: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();

    for c in content.chars() {
        
        let entry = frequencies.entry(c).or_insert(0);
        *entry += 1;
    }

    frequencies
}


// 2.5 & 2.6: Consolidated File Analysis and Error Handling
pub fn analyze_file(file_path: &Path, cancellation_token: &AtomicBool) -> FileAnalysis {
    let start_time = Instant::now();
    let filename = file_path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let mut errors: Vec<ProcessingError> = Vec::new();
    
    // Default stats in case of failure
    let mut stats = FileStats {
        word_count: 0,
        line_count: 0,
        char_frequencies: HashMap::new(),
        size_bytes: 0,
    };

    if cancellation_token.load(Ordering::SeqCst) {
        errors.push(ProcessingError::Cancelled);
        return FileAnalysis {
            filename,
            stats,
            errors,
            processing_time: start_time.elapsed(),
        };
    }

    // --- File System Handling and Size Stats (2.5) ---
    match fs::metadata(file_path) {
        Ok(metadata) => {
            stats.size_bytes = metadata.len();
        },
        Err(e) => {
            
            errors.push(ProcessingError::IoError(format!("Metadata error: {}", e)));
            
            return FileAnalysis {
                filename,
                stats,
                errors,
                processing_time: start_time.elapsed(),
            };
        }
    }

    
    match fs::read_to_string(file_path) {
        Ok(content) => {
            if cancellation_token.load(Ordering::SeqCst) {
                // If cancelled, log the cancellation error and return immediately.
                errors.push(ProcessingError::Cancelled);
                return FileAnalysis {
                    filename,
                    stats,
                    errors,
                    processing_time: start_time.elapsed(),
                };
            }
            // --- Run Analyzers ---
            let (line_count, word_count) = analyze_text_counts(&content);
            stats.line_count = line_count;
            stats.word_count = word_count;
            stats.char_frequencies = analyze_char_frequencies(&content);
        },
        Err(e) => {
            
            errors.push(ProcessingError::IoError(format!("Read error: {}", e)));
        }
    }

    
    FileAnalysis {
        filename,
        stats,
        errors,
        processing_time: start_time.elapsed(),
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_text_counts_accuracy() {
        let content = "The quick brown fox jumps.\nOver the lazy dog.\n";
        let (lines, words) = super::analyze_text_counts(content);

        
        assert_eq!(lines, 2, "Line count failed.");
        assert_eq!(words, 9, "Word count failed (5 + 4 = 9).");
    }

    
    #[test]
    fn test_text_counts_edge_cases() {
        let empty_content = "";
        let (lines_e, words_e) = super::analyze_text_counts(empty_content);
        assert_eq!(lines_e, 0, "Empty content line count failed.");
        assert_eq!(words_e, 0, "Empty content word count failed.");

        let whitespace_content = "   \n \t \n";
        let (lines_w, words_w) = super::analyze_text_counts(whitespace_content);
        
        assert_eq!(lines_w, 2, "Whitespace line count failed.");
        assert_eq!(words_w, 0, "Whitespace word count failed.");
    }

    
    #[test]
    fn test_char_frequency_accuracy() {
        let content = "aaabbc!";
        let freqs = super::analyze_char_frequencies(content);

        assert_eq!(*freqs.get(&'a').unwrap_or(&0), 3, "Frequency of 'a' incorrect.");
        assert_eq!(*freqs.get(&'b').unwrap_or(&0), 2, "Frequency of 'b' incorrect.");
        assert_eq!(*freqs.get(&'c').unwrap_or(&0), 1, "Frequency of 'c' incorrect.");
        assert_eq!(*freqs.get(&'!').unwrap_or(&0), 1, "Frequency of '!' incorrect.");
        assert_eq!(freqs.get(&'z'), None, "Frequency should not contain 'z'.");
    }
}