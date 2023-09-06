use std::fs;

fn main() {
    // Get the current directory
    let current_dir = std::env::current_dir().expect("Unable to get current directory");

    // Define the path to the 'files' folder
    let files_folder_path = current_dir.join("files");

    // Read the 'files' folder
    if let Ok(entries) = fs::read_dir(&files_folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if let Some(file_name) = file_path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        // Check if the file name contains 'en'
                        if file_name_str.contains("en") {
                            // Create a new folder if it doesn't exist
                            let new_folder_path = current_dir.join("en_files");
                            if !new_folder_path.exists() {
                                fs::create_dir(&new_folder_path)
                                    .expect("Unable to create new folder");
                            }

                            // Move the file to the new folder
                            let new_file_path = new_folder_path.join(file_name);
                            if let Err(err) = fs::rename(&file_path, &new_file_path) {
                                eprintln!("Error moving file: {}", err);
                            } else {
                                println!(
                                    "Moved {} to {}",
                                    file_name_str,
                                    new_folder_path.display()
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
