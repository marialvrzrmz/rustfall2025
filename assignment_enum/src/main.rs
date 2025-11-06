use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum DownloadError {
    ApiError(String),
    NetworkError(String),
    FileError(String),
}

fn fetch_random_dog_image() -> Result<DogImage, DownloadError> {
    let url = "https://dog.ceo/api/breeds/image/random";

    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => Ok(dog_image),
                    Err(e) => Err(DownloadError::ApiError(format!("JSON parse failed: {}", e))),
                }
            } else {
                Err(DownloadError::ApiError(format!("HTTP error: {}", response.status())))
            }
        }
        Err(e) => Err(DownloadError::NetworkError(format!("Request failed: {}", e))),
    }
}

fn download_image(url: &str, filename: &str) -> Result<(), DownloadError> {
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                let mut reader = response.into_reader();
                let mut buffer = Vec::new();
                std::io::copy(&mut reader, &mut buffer)
                    .map_err(|e| DownloadError::NetworkError(format!("Read failed: {}", e)))?;

                let mut file = fs::File::create(filename)
                    .map_err(|e| DownloadError::FileError(format!("File create failed: {}", e)))?;
                file.write_all(&buffer)
                    .map_err(|e| DownloadError::FileError(format!("Write failed: {}", e)))?;
                Ok(())
            } else {
                Err(DownloadError::ApiError(format!(
                    "Failed to download image: HTTP {}",
                    response.status()
                )))
            }
        }
        Err(e) => Err(DownloadError::NetworkError(format!("Request failed: {}", e))),
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("🐕 Dog Image Downloader");
    println!("========================\n");

    let image_folder = "images";
    if !Path::new(image_folder).exists() {
        fs::create_dir(image_folder)?;
    }

    for i in 1..=5 {
        println!("Fetching dog image #{}...", i);

        match fetch_random_dog_image() {
            Ok(dog_image) => {
                println!("Got image URL: {}", dog_image.message);

                let file_path = format!("{}/dog_{}.jpg", image_folder, i);
                match download_image(&dog_image.message, &file_path) {
                    Ok(_) => println!("Saved to {}\n", file_path),
                    Err(e) => println!("Download error: {:?}\n", e),
                }
            }
            Err(e) => println!("API error: {:?}\n", e),
        }
    }

    println!("All done! Check your 'images/' folder.");
    Ok(())
}
