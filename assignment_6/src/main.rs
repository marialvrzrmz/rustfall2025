use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = File::create(filename).expect("Unable to create file");

    for book in books {
        // Write each book in the format: title,author,year
        writeln!(file, "{},{},{}", book.title, book.author, book.year)
            .expect("Unable to write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut books = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                let title = parts[0].to_string();
                let author = parts[1].to_string();
                let year = parts[2].trim().parse::<u16>().unwrap_or(0);
                books.push(Book { title, author, year });
            }
        }
    }

    books
}

fn main() {
    let books = vec![
        Book { title: "Fahrenheit 451".to_string(), author: "Ray Bradbury".to_string(), year: 1953 },
        Book { title: "The Cask of Amontillado".to_string(), author: "Edgar Allan Poe".to_string(), year: 1846 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}
