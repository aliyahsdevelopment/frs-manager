#[cfg(target_os = "windows")]
mod utils;
mod models;

use std::{fs::File, path::Path};

#[cfg(target_os = "windows")]
fn update_env_path(download_path: &Path) {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    match std::env::var("PATH") {
        Ok(env_path) => {
            let frs_env_path = download_path.join("bin").display().to_string();

            if !env_path.contains(frs_env_path.as_str()) {
                let new_env_path = format!("{};{}", frs_env_path, env_path);
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                let (env, _) = hkcu.create_subkey("Environment").unwrap();
                env.set_value("Path", &new_env_path).unwrap();
                println!("FRS was added to the env path");
            } else {
                println!("FRS already exists in env path");
            }
        }

        Err(err) => {
            println!("Couldnt fetch windows env variables: {}", err);
        }
    };
}

#[cfg(not(target_os = "windows"))]
fn update_env_path(_download_path: &Path) {}

#[tokio::main]
#[cfg(target_os = "windows")]
pub async fn main() {
    use std::fs::read_to_string;
    use std::io::{prelude::*, Cursor};
    use tokio::fs::create_dir;
    use crate::utils::{get_download_path, wait_before_close, get_download_data};

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
                    if !version_id_path.exists() {
                        let version_file = File::create(version_id_path.clone());
                        match version_file {
                            Ok(mut version_file) => {
                                match version_file.write(download_data.version.to_string().as_bytes()) {
                                    Ok(_) => {
                                        println!("Wrote to version file");
                                    }

                                    Err(err) => {
                                        println!(
                                            "Error occured whilst writing to version file: {}",
                                            err
                                        );
                                    }
                                }
                            }

                            Err(err) => {
                                println!("Error occured whilst creating version file: {}", err);
                            }
                        }
                    } else {
                        println!("Version file already exists, no need to initialize")
                    }

                    let current_version_value: i32 = read_to_string(version_id_path.clone())
                        .expect("Couldnt read version id")
                        .parse()
                        .expect("Couldnt convert version id to i32");

                    let frs_path = bin_path.clone().join("frs.exe");
                    let installer_path = download_path.join("installer.exe");
                    let uninstaller_path = download_path.join("uninstaller.exe");

                    if !frs_path.exists() || current_version_value != download_data.version {
                        match reqwest::get(download_data.frs_exe).await {
                            Ok(frs_exe_data) => match frs_exe_data.bytes().await {
                                Ok(frs_exe_data_bytes) => {
                                    let frs_file = File::create(frs_path);

                                    match frs_file {
                                        Ok(mut frs_file) => {
                                            match std::io::copy(
                                                &mut Cursor::new(frs_exe_data_bytes),
                                                &mut frs_file,
                                            ) {
                                                Ok(_) => {
                                                    println!("Wrote to frs exe file");
                                                }

                                                Err(err) => {
                                                    println!("Error occured whilst writing to frs exe file: {}", err);
                                                }
                                            }
                                        }

                                        Err(err) => {
                                            println!("Error whilst fetching frs body data: {}", err);
                                        }
                                    }
                                }

                                Err(err) => {
                                    println!("Error whilst fetching frs body data: {}", err);
                                }
                            },

                            Err(err) => {
                                println!("Error whilst fetching frs data: {}", err);
                            }
                        }
                    } else {
                        println!("Version matches & frs exe already exists, no need to update the file")
                    }

                    if !installer_path.exists() || current_version_value != download_data.version {
                        match reqwest::get(download_data.installer_exe).await {
                            Ok(installer_exe_data) => match installer_exe_data.bytes().await {
                                Ok(installer_exe_data_bytes) => {
                                    let installer_file = File::create(installer_path);

                                    match installer_file {
                                        Ok(mut installer_file) => {
                                            match std::io::copy(
                                                &mut Cursor::new(installer_exe_data_bytes),
                                                &mut installer_file,
                                            ) {
                                                Ok(_) => {
                                                    println!("Wrote to installer exe file");
                                                }

                                                Err(err) => {
                                                    println!("Error occured whilst writing to installer exe file: {}", err);
                                                }
                                            }
                                        }

                                        Err(err) => {
                                            println!("Error whilst fetching installer body data: {}", err);
                                        }
                                    }
                                }

                                Err(err) => {
                                    println!("Error whilst fetching installer body data: {}", err);
                                }
                            },

                            Err(err) => {
                                println!("Error whilst fetching installer data: {}", err);
                            }
                        }
                    } else {
                        println!("Version matches & installer exe already exists, no need to update the file")
                    }

                    if !uninstaller_path.exists() || current_version_value != download_data.version {
                        match reqwest::get(download_data.uninstaller_exe).await {
                            Ok(uninstaller_exe_data) => match uninstaller_exe_data.bytes().await {
                                Ok(uninstaller_exe_data_bytes) => {
                                    let uninstaller_file = File::create(uninstaller_path);

                                    match uninstaller_file {
                                        Ok(mut uninstaller_file) => {
                                            match std::io::copy(
                                                &mut Cursor::new(uninstaller_exe_data_bytes),
                                                &mut uninstaller_file,
                                            ) {
                                                Ok(_) => {
                                                    println!("Wrote to uninstaller exe file");
                                                }

                                                Err(err) => {
                                                    println!("Error occured whilst writing to uninstaller exe file: {}", err);
                                                }
                                            }
                                        }

                                        Err(err) => {
                                            println!("Error whilst fetching uninstaller body data: {}", err);
                                        }
                                    }
                                }

                                Err(err) => {
                                    println!("Error whilst fetching uninstaller body data: {}", err);
                                }
                            },

                            Err(err) => {
                                println!("Error whilst fetching uninstaller data: {}", err);
                            }
                        }
                    } else {
                        println!("Version matches & uninstaller exe already exists, no need to update the file")
                    }

                    if current_version_value != download_data.version {
                        let version_file = File::create(version_id_path.clone());
                        match version_file {
                            Ok(mut version_file) => {
                                match version_file.write(download_data.version.to_string().as_bytes()) {
                                    Ok(_) => {
                                        println!("Wrote to version file");
                                    }

                                    Err(err) => {
                                        println!(
                                            "Error occured whilst writing to version file: {}",
                                            err
                                        );
                                    }
                                }
                            }

                            Err(err) => {
                                println!("Error occured whilst writing to version file: {}", err);
                            }
                        }
                    } else {
                        println!("Updated/downloaded files, but version number is already correct, so leaving it as it is")
                    }

                    update_env_path(download_path);
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