#[cfg(target_os = "windows")]
mod utils;
#[cfg(target_os = "windows")]
mod models;

#[tokio::main]
#[cfg(target_os = "windows")]
pub async fn main() {
    use std::path::Path;
    use tokio::fs::create_dir;
    use crate::utils::{get_download_path, wait_before_close, get_download_data, update_version_file, get_version_value, write_file_from_link, add_env_var};
    
    let download_data = get_download_data().await;
    let raw_download_path = get_download_path();

    match download_data {
        Ok(download_data) => {
            match raw_download_path {
                Ok(raw_download_path) => {
                    let download_path = Path::new(raw_download_path.as_str());
                    let bin_path = download_path.join("bin");

                    if !download_path.exists() {
                        create_dir(download_path)
                            .await
                            .expect("Couldnt create download path");
                        println!("Created folder for frs-manager");
                    } else {
                        println!("Folder already exists");
                    }

                    if !bin_path.clone().exists() {
                        create_dir(bin_path.clone())
                            .await
                            .expect("Couldnt create bin path");
                        println!("Created bin folder for frs-manager");
                    } else {
                        println!("Bin folder already exists");
                    }

                    let version_id_path = download_path.join("version_id");
                    let _ = update_version_file(version_id_path.clone(), download_data.version);

                    let current_version_value: i32 = get_version_value(version_id_path.clone()).expect("Couldnt fetch version value");

                    let frs_path = bin_path.clone().join("frs.exe");
                    let installer_path = download_path.join("installer.exe");
                    let uninstaller_path = download_path.join("uninstaller.exe");

                    if !frs_path.exists() || current_version_value != download_data.version {
                        let _ = write_file_from_link(frs_path, download_data.frs_exe);
                    } else {
                        println!("Version matches & frs exe already exists, no need to update the file")
                    }

                    if !installer_path.exists() || current_version_value != download_data.version {
                        let _ = write_file_from_link(installer_path, download_data.installer_exe);
                    } else {
                        println!("Version matches & installer exe already exists, no need to update the file")
                    }

                    if !uninstaller_path.exists() || current_version_value != download_data.version {
                        let _ = write_file_from_link(uninstaller_path, download_data.uninstaller_exe);
                    } else {
                        println!("Version matches & uninstaller exe already exists, no need to update the file")
                    }

                    if current_version_value != download_data.version {
                        let _ = update_version_file(version_id_path.clone(), download_data.version);
                    } else {
                        println!("Updated/downloaded files, but version number is already correct, so leaving it as it is")
                    }

                    let _ = add_env_var(download_path.join("bin"));
                }

                Err(err) => {
                    println!("Error whilst fetching download path: {}", err);
                }
            }
        }

        Err(err) => {
            println!("Error whilst fetching download url: {}", err);
        }
    }

    wait_before_close();
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("This installer can only run on windows.")
}