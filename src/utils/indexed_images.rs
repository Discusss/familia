use std::collections::HashMap;

// Returns a hashmap of (String -> u32) where the String is the type name and the u32 is the ammount of images
pub fn index_image_sizes() -> HashMap<String, Vec<String>> {

    // If there is no 'assets' directory, throw an error
    if !std::path::Path::new("assets").exists() {
        let files_in_same_dir = std::fs::read_dir(".").unwrap();
        let files: Vec<String> = files_in_same_dir
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect();
        panic!("No 'assets' directory found. Files in the current directory: {:?}", files);
    }

    // For every directory on the 'assets' directory, check if it is a directory
    let mut image_sizes: HashMap<String, Vec<String>> = HashMap::new();
    for entry in std::fs::read_dir("assets").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // If it is a directory, get the name of all the files that ends with .png, .jpg, .jpeg, .gif and .webp
            let mut images: Vec<String> = Vec::new();
            for file in std::fs::read_dir(path.clone()).unwrap() {
                let file = file.unwrap();
                let path = file.path();
                if path.is_file() {
                    let ext = path.extension().unwrap().to_str().unwrap().to_lowercase();
                    if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "gif" || ext == "webp" {
                        images.push(path.file_name().unwrap().to_str().unwrap().to_string());
                    }
                }
            }
            // If there are images, add them to the hashmap
            if !images.is_empty() {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                image_sizes.insert(name, images);
            }
        }
    }

    // If there are no images, throw an error
    if image_sizes.is_empty() {
        panic!("No images found");
    }

    // Return the hashmap
    image_sizes
}