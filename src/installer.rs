#[cfg(target_os = "windows")]
mod utils;

use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ReleaseAsset {
    pub browser_download_url: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReleaseResponse {
    pub url: String,
    pub assets: Vec<ReleaseAsset>,
    pub id: i32,
}

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

// (frs.exe, installer.exe, id, uninstaller.exe)
struct DownloadData {
    pub frs_exe: String,
    pub installer_exe: String,
    pub uninstaller_exe: String,
    pub version: i32,
}

#[cfg(target_os = "windows")]
async fn get_download_data() -> Result<DownloadData, String> {
    let reqwest_client = reqwest::Client::builder().user_agent("FRS-Manager").build();

    match reqwest_client {
        Ok(reqwest_client) => {
            match reqwest_client
                .get("https://api.github.com/repos/Z3rio/frs-manager/releases/latest")
                .send()
                .await
            {
                Ok(resp) => match resp.json::<ReleaseResponse>().await {
                    Ok(json) => {
                        let frs_exe_asset =
                            json.assets.clone().into_iter().find(|a| a.name == "frs.exe");

                        match frs_exe_asset {
                            Some(frs_exe_asset) => {
                                let installer_exe_asset =
                                    json.assets.clone().into_iter().find(|a| a.name == "installer.exe");

                                match installer_exe_asset {
                                    Some(installer_exe_asset) => {
                                        let uninstaller_exe_asset =
                                            json.assets.clone().into_iter().find(|a| a.name == "uninstaller.exe");
        
                                        match uninstaller_exe_asset {
                                            Some(uninstaller_exe_asset) => Ok(DownloadData {
                                                frs_exe: frs_exe_asset.browser_download_url,
                                                installer_exe: installer_exe_asset.browser_download_url,
                                                uninstaller_exe: uninstaller_exe_asset.browser_download_url,
                                                version: json.id,
                                            }),

                                            None => Err(String::from(
                                                "Could not find uninstaller in assets list",
                                            ))
                                        }
                                    }

                                    None => Err(String::from(
                                        "Could not find installer in assets list",
                                    ))
                                }
                            }

                            None => Err(String::from(
                                "Could not find frs in assets list",
                            ))
                        }
                    }

                    Err(err) => Err(err.to_string())
                },

                Err(err) => Err(err.to_string())
            }
        }

        Err(err) => Err(err.to_string())
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_download_data() -> Result<DownloadData, String> {
    return Err(String::from("This function doesnt support linux"));
}

#[tokio::main]
#[cfg(target_os = "windows")]
async fn main() {
    use std::fs::read_to_string;
    use std::io::{prelude::*, Cursor};
    use tokio::fs::create_dir;
    use utils::{get_download_path, wait_before_close};

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