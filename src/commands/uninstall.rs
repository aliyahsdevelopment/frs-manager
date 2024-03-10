use std::path::Path;
use crate::utils::{cleanup_files, get_download_path, remove_env_var};

pub fn handler() {
    let raw_download_path = get_download_path();
    
    match raw_download_path {
        Ok(raw_download_path) => {
            let download_path = Path::new(raw_download_path.as_str());

            match remove_env_var(download_path) {
                Ok(_) => {
                    println!("Successfully removed env variable");
                }

                Err(err) => {
                    println!("Failed to remove env variable: {}", err);
                }
            }

            match cleanup_files(download_path) {
                Ok(_) => {
                    println!("Successfully removed files");
                }

                Err(err) => {
                    println!("Failed to remove frs files: {}", err);
                }
            }
        }

        Err(err) => {
            println!("Failed to fetch download path: {}", err);
        }
    }
}