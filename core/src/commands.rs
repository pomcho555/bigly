use super::error;
use super::hazard;

use std::fs;
use std::io::{BufRead, BufReader};
use utils::app_config::AppConfig;
use utils::error::Result;

/// Show the configuration file
pub fn hazard() -> Result<()> {
    // Generate, randomly, True or False
    let random_hazard: bool = hazard::generate_hazard()?;

    if random_hazard {
        println!("You got it right!");
    } else {
        println!("You got it wrong!");
    }

    Ok(())
}

/// Show the configuration file
pub fn config() -> Result<()> {
    let config = AppConfig::fetch()?;
    println!("{config:#?}");

    Ok(())
}

/// Simulate an error
pub fn simulate_error() -> Result<()> {
    // Log this Error simulation
    info!("We are simulating an error");

    // Simulate an error
    error::simulate_error()?;

    // We should never get here...
    Ok(())
}

/// Search for a term in files
pub fn search(search_term: &str, target_file: Option<&str>) -> Result<()> {
    if let Some(file_path) = target_file {
        // Search in specific file
        search_in_file(search_term, file_path)?;
    } else {
        // Search in all files in current directory
        search_in_directory(search_term, ".")?;
    }
    Ok(())
}

/// Search for a term in a specific file
fn search_in_file(search_term: &str, file_path: &str) -> Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(search_term) {
            println!("{}:{}:{}", file_path, line_number + 1, line);
        }
    }
    Ok(())
}

/// Search for a term in all files in a directory
fn search_in_directory(search_term: &str, dir_path: &str) -> Result<()> {
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // Skip binary files and hidden files
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.starts_with('.') {
                    continue;
                }
            }

            // Try to search in the file, skip if it's binary or unreadable
            if let Ok(()) = search_in_file(search_term, path.to_str().unwrap_or("")) {
                // File was successfully searched
            }
        } else if path.is_dir() {
            // Skip hidden directories and target directory
            if let Some(dir_name) = path.file_name() {
                let dir_name_str = dir_name.to_string_lossy();
                if dir_name_str.starts_with('.') || dir_name_str == "target" {
                    continue;
                }
            }

            // Recursively search subdirectories
            if let Some(path_str) = path.to_str() {
                search_in_directory(search_term, path_str)?;
            }
        }
    }
    Ok(())
}
