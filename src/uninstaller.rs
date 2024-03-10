#[cfg(target_os = "windows")]
mod utils;
mod models;

#[tokio::main]
#[cfg(target_os = "windows")]
async fn main() {
    use std::{io::{stdin, stdout, Write}, path::Path};
    use utils::{get_download_path, wait_before_close, remove_env_var, cleanup_files};

    let raw_download_path = get_download_path();

    match raw_download_path {
        Ok(raw_download_path) => {
            let mut raw_input = String::new();

            while raw_input != "Y" && raw_input != "N" {                
                println!("Are you sure you wish to uninstall FRS-Manager? Y/N");

                let _ = stdout().flush();
                stdin().read_line(&mut raw_input).expect("Did not enter anything");
                if let Some('\n') = raw_input.chars().next_back() {
                    raw_input.pop();
                }
                if let Some('\r') = raw_input.chars().next_back() {
                    raw_input.pop();
                }

                raw_input = raw_input.to_uppercase();
                if raw_input != "Y" && raw_input != "N" {
                    println!("Invalid input");
                    raw_input = String::new();
                }
            }

            if raw_input == "Y" {
                println!("Uninstalling FRS...");
                let download_path = Path::new(raw_download_path.as_str());

                remove_env_var(download_path);

                match cleanup_files(download_path) {
                    Ok(_) => {
                        println!("Successfully removed files");
                    }

                    Err(err) => {
                        println!("Failed to remove frs files: {}", err);
                    }
                }
            } else {
                println!("Cancelling uninstallation process...");
            }
        }

        Err(err) => {
            println!("Error whilst fetching download path: {}", err);
        }
    }

    wait_before_close();
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("This uninstaller can only run on windows.")
}