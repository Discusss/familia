use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use rand::Rng;
use serde_json::Value;

// const API_URL: &str = "https://random-d.uk/api/v2/random";
const API_URL: &str = "https://randomfox.ca/floof";
const IMAGE_URL_FIELD: &str = "image";
const USER_AGENT: &str = "BulkImageDownloader/1.0";
const TIMEOUT_SECS: u64 = 30;

/// Make HTTP GET request and return response as string
fn http_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
        .build()?;

    let response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("HTTP error code: {}", response.status()).into());
    }

    Ok(response.text()?)
}

/// Download file from URL to destination
fn download_file(url: &str, dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
        .build()?;

    let response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("Download HTTP error code: {}", response.status()).into());
    }

    let bytes = response.bytes()?;
    let mut file = File::create(dest_path)?;
    file.write_all(&bytes)?;

    Ok(())
}

/// Extract filename from URL
fn get_filename_from_url(url: &str) -> Option<String> {
    url.rsplit('/').next().map(|s| s.to_string())
}

/// Get file extension from filename
fn get_extension(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| format!(".{}", ext))
        .unwrap_or_default()
}

/// Generate random string for temporary filename
fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::rng();

    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Save downloaded filenames to a tracking file
fn save_downloaded_list(folder: &Path, downloaded: &HashSet<String>) -> io::Result<()> {
    let file_path = folder.join(".downloaded_images.txt");
    let mut file = File::create(file_path)?;

    for name in downloaded {
        writeln!(file, "{}", name)?;
    }

    Ok(())
}

/// Load previously downloaded filenames
fn load_downloaded_list(folder: &Path) -> HashSet<String> {
    let file_path = folder.join(".downloaded_images.txt");
    let mut downloaded = HashSet::new();

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if !line.trim().is_empty() {
                downloaded.insert(line);
            }
        }
    }

    downloaded
}

/// Find the highest numbered file in the folder
fn find_highest_number(folder: &Path) -> i32 {
    let mut highest = 0;

    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                // Skip hidden files
                if file_name.starts_with('.') {
                    continue;
                }

                // Extract number from filename (e.g., "123.jpg" -> 123)
                if let Some(name_without_ext) = file_name.split('.').next() {
                    if let Ok(num) = name_without_ext.parse::<i32>() {
                        if num > highest {
                            highest = num;
                        }
                    }
                }
            }
        }
    }

    highest
}

/// Validate folder name for Windows
fn is_valid_windows_folder_name(name: &str) -> bool {
    const INVALID_CHARS: &str = "<>:\"|?*";

    for c in name.chars() {
        if INVALID_CHARS.contains(c) || (c as u32) < 32 {
            return false;
        }
    }

    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing...\n");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    let (num_images, dest_folder) = if args.len() == 3 {
        let num = args[1].parse::<i32>().map_err(|_| {
            eprintln!("Error: Invalid arguments.");
            eprintln!("Usage: {} <number_of_images> <destination_folder>", args[0]);
            "Invalid number"
        })?;
        (num, args[2].clone())
    } else {
        // Interactive mode
        print!("Enter the number of images to download: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let num = input.trim().parse::<i32>().map_err(|_| "Invalid number")?;

        print!("Enter the destination folder name: ");
        io::stdout().flush()?;

        let mut folder = String::new();
        io::stdin().read_line(&mut folder)?;
        (num, folder.trim().to_string())
    };

    // Validate inputs
    if num_images <= 0 {
        eprintln!("Error: Number of images must be greater than 0.");
        return Err("Invalid input".into());
    }

    if dest_folder.is_empty() {
        eprintln!("Error: Destination folder cannot be empty.");
        return Err("Invalid input".into());
    }

    // Validate folder name for Windows
    if !is_valid_windows_folder_name(&dest_folder) {
        eprintln!("Error: Invalid folder name. Contains illegal characters.");
        return Err("Invalid folder name".into());
    }

    let folder_path = std::env::current_dir()?.join(&dest_folder);

    // Check if folder exists and has items
    if folder_path.exists() {
        let has_items = fs::read_dir(&folder_path)?
            .any(|entry| {
                entry.map(|e| e.file_name() != ".downloaded_images.txt").unwrap_or(false)
            });

        if has_items {
            println!("Warning: Folder '{}' already exists and contains items!", dest_folder);
            print!("Do you want to continue? (y/n): ");
            io::stdout().flush()?;

            let mut response = String::new();
            io::stdin().read_line(&mut response)?;

            if !response.trim().eq_ignore_ascii_case("y") {
                println!("Operation cancelled.");
                return Ok(());
            }
        }
    } else {
        // Create folder
        fs::create_dir_all(&folder_path)?;
        println!("Created folder: {}", folder_path.display());
    }

    // Load previously downloaded images
    let mut downloaded_images = load_downloaded_list(&folder_path);
    println!("Loaded {} previously downloaded image(s).", downloaded_images.len());

    // Store temporary filenames and their extensions
    let mut temp_files: Vec<(String, String)> = Vec::new();

    println!("\nStarting download of {} image(s)...", num_images);

    let mut success_count = 0;
    let mut attempts = 0;
    let max_attempts = num_images * 3; // Prevent infinite loops

    while success_count < num_images && attempts < max_attempts {
        attempts += 1;
        print!("\n[{}/{}] ", success_count + 1, num_images);
        io::stdout().flush()?;

        // Make API request
        let api_response = match http_get(API_URL) {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("API request failed: {}", e);
                continue;
            }
        };

        // Parse JSON response
        let api_data: Value = match serde_json::from_str(&api_response) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("JSON parsing error: {}", e);
                continue;
            }
        };

        let image_url = match api_data.get(IMAGE_URL_FIELD).and_then(|v| v.as_str()) {
            Some(url) => url,
            None => {
                eprintln!("API response missing '{}' field", IMAGE_URL_FIELD);
                continue;
            }
        };

        let original_filename = match get_filename_from_url(image_url) {
            Some(name) => name,
            None => {
                eprintln!("Failed to extract filename from URL: {}", image_url);
                continue;
            }
        };

        // Check if already downloaded
        if downloaded_images.contains(&original_filename) {
            println!("Image '{}' already downloaded, skipping...", original_filename);
            continue;
        }

        let extension = get_extension(&original_filename);
        let temp_filename = format!("{}{}", generate_random_string(16), extension);
        let temp_file_path = folder_path.join(&temp_filename);

        println!("Downloading: {} -> {}", original_filename, temp_filename);

        // Download the image
        if let Err(e) = download_file(image_url, &temp_file_path) {
            eprintln!("Download failed: {}", e);
            continue;
        }

        // Verify file exists and has content
        match fs::metadata(&temp_file_path) {
            Ok(metadata) if metadata.len() > 0 => {
                println!("Success! Downloaded {} bytes.", metadata.len());

                // Mark as downloaded
                downloaded_images.insert(original_filename.clone());
                temp_files.push((temp_filename, extension));
                success_count += 1;

                // Save the downloaded list periodically
                let _ = save_downloaded_list(&folder_path, &downloaded_images);
            }
            _ => {
                eprintln!("Downloaded file is invalid or empty");
                let _ = fs::remove_file(&temp_file_path);
                continue;
            }
        }
    }

    if success_count < num_images {
        eprintln!("\nWarning: Only downloaded {} out of {} images.", success_count, num_images);
    }

    // Rename files to numbered format
    if success_count > 0 {
        println!("\nRenaming files to numbered format...");

        // Find the highest existing number to avoid conflicts
        let highest_number = find_highest_number(&folder_path);
        let mut next_number = highest_number + 1;

        for (i, (temp_filename, extension)) in temp_files.iter().enumerate() {
            let old_path = folder_path.join(temp_filename);
            let new_filename = format!("{}{}", next_number, extension);
            let new_path = folder_path.join(&new_filename);

            match fs::rename(&old_path, &new_path) {
                Ok(_) => {
                    println!("Renamed: {} -> {}", temp_filename, new_filename);
                    next_number += 1;
                },
                Err(e) => eprintln!("Failed to rename {}: {}", temp_filename, e),
            }
        }
    }

    // Save final downloaded list
    save_downloaded_list(&folder_path, &downloaded_images)?;

    println!("\n=== Download Complete ===");
    println!("Successfully downloaded: {} image(s)", success_count);
    println!("Destination: {}", folder_path.display());

    Ok(())
}
