#[cfg(target_os = "windows")]
mod utils;

#[tokio::main]
#[cfg(target_os = "windows")]
async fn main() {
    use std::{fs::remove_dir_all, io::{stdin, stdout, Write}, path::Path};
    use utils::{get_download_path, wait_before_close};
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

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

                if download_path.exists() {
                    match remove_dir_all(download_path) {
                        Ok(_) => {
                            println!("Removed FRS directory");
                        }

                        Err(err) => {
                            println!("Failed to remove FRS directory: {}", err);
                        }
                    }
                } else {
                    println!("FRS directory already doesnt exist");
                }

                match std::env::var("PATH") {
                    Ok(env_path) => {
                        let frs_env_path = download_path.join("bin").display().to_string();
                        
                        if env_path.contains(frs_env_path.as_str()) == true {
                            let splits = env_path.split(";");

                            let filtered = splits.filter(|&s| s != frs_env_path.as_str());
                            let joined = filtered.map(|x| x).collect::<Vec<&str>>().join(";");
                            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                            let (env, _) = hkcu.create_subkey("Environment").unwrap();
                            env.set_value("Path", &joined).unwrap();
                            println!("FRS was removed from the env path");
                        } else {
                            println!("FRS already doesnt exist in env path");
                        }
                    }
            
                    Err(err) => {
                        println!("Couldnt fetch windows env variables: {}", err.to_string());
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