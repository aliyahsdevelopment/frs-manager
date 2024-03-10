use crate::models::{DownloadData, ReleaseResponse};
use std::{fs::{read_to_string, File}, path::{Path, PathBuf}};

#[cfg(target_os = "windows")]
pub fn cleanup_files(download_path: &Path) -> Result<(), String> {
    use std::fs::{remove_dir_all, remove_file};

    if download_path.exists() {
        match houdini::disappear() {
            Ok(_) => {
                match remove_file(download_path.join("installer.exe")) {
                    Ok(_) => {}

                    Err(err) => {
                        return Err(format!("Failed to remove installer file: {}", err));
                    }
                }

                match remove_file(download_path.join("version_id")) {
                    Ok(_) => {},

                    Err(err) => {
                        return Err(format!("Failed to remove version file: {}", err))
                    }
                }

                match remove_dir_all(download_path.join("bin")) {
                    Ok(_) => {},

                    Err(err) => {
                        return Err(format!("Failed to remove bin folder: {}", err))
                    }
                }

                Ok(())
            }

            Err(err) => Err(format!("Failed to remove uninstaller: {}", err))
        }
    } else {
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub fn cleanup_files(_download_path: &Path) -> Result<(), String> {
    return Err(String::from("This function doesnt support linux"));
}

#[cfg(target_os = "windows")]
pub fn remove_env_var(download_path: &Path) -> Result<(), String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    return match std::env::var("PATH") {
        Ok(env_path) => {
            let frs_env_path = download_path.join("bin").display().to_string();
            
            if env_path.contains(frs_env_path.as_str()) {
                let splits = env_path.split(';');

                let filtered = splits.filter(|&s| s != frs_env_path.as_str());
                let joined = filtered.collect::<Vec<&str>>().join(";");
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                let (env, _) = hkcu.create_subkey("Environment").unwrap();
                env.set_value("Path", &joined).unwrap();
                println!("FRS was removed from the env path");
                Ok(())
            } else {
                println!("FRS already doesnt exist in env path");
                Ok(())
            }
        }

        Err(err) => {
            Err(format!("Couldnt fetch windows env variables: {}", err))
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn add_env_var(_download_path: &Path) -> Result<(), String> {
    return Err(String::from("This function doesnt support linux"));
}

#[cfg(target_os = "windows")]
pub fn add_env_var(frs_env_path: PathBuf) -> Result<(), String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let frs_env_path = frs_env_path.display().to_string();

    return match std::env::var("PATH") {
        Ok(env_path) => {            
            if !env_path.contains(frs_env_path.as_str()) {
                let splits = env_path.split(';');
                let mut splits_vec = splits.collect::<Vec<&str>>();
                
                splits_vec.push(frs_env_path.as_str());

                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                let (env, _) = hkcu.create_subkey("Environment").unwrap();
                env.set_value("Path", &splits_vec.join(";")).unwrap();
                Ok(())
            } else {
                Ok(())
            }
        }

        Err(err) => {
            Err(err.to_string())
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn remove_env_var(_download_path: &Path) -> Result<(), String> {
    return Err(String::from("This function doesnt support linux"));
}


#[cfg(target_os = "windows")]
pub fn get_download_path() -> Result<String, String> {
    return match std::env::var("ProgramFiles") {
        Ok(program_files_path) => Ok(Path::new(program_files_path.as_str())
            .join("FRS_Manager")
            .display()
            .to_string()
        ),

        Err(err) => Err(err.to_string())
    };
}

#[cfg(not(target_os = "windows"))]
pub fn get_download_path() -> Result<String, String> {
    return Err(String::from("This function doesnt support linux"));
}

pub fn wait_before_close() {
    use std::io::{stdin, stdout, prelude::*};

    println!("Press any key to continue...");
    stdout().flush().unwrap();

    // Read a single byte and discard
    let _ = stdin().read(&mut [0u8]).unwrap();
}

#[cfg(target_os = "windows")]
pub async fn get_download_data() -> Result<DownloadData, String> {
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

pub fn update_version_file(version_id_path: PathBuf, version: i32) -> Result<(), String> {
    use std::io::prelude::Write;
    
    let version_file = File::create(version_id_path);
    return match version_file {
        Ok(mut version_file) => {
            return match version_file.write(version.to_string().as_bytes()) {
                Ok(_) => Ok(()),

                Err(err) => Err(format!(
                    "Error occured whilst writing to version file: {}",
                    err
                ))
            }
        }

        Err(err) => Err(format!("Error occured whilst creating version file: {}", err))
    }
}

pub fn get_version_value(version_id_path: PathBuf) -> Result<i32, String> {    
    let current_version_value = read_to_string(version_id_path.clone());

    match current_version_value {
        Ok(current_version_value) => {
            let value = current_version_value.parse::<i32>();

            match value {
                Ok(value) => {
                    return Ok(value)
                }

                Err(err) => {
                    return Err(err.to_string())
                }
            }
        }

        Err(err) => {
            return Err(err.to_string());
        }
    }
}

pub async fn write_file_from_link(file_path: PathBuf, link: String) -> Result<(), String> {
    use std::io::Cursor;

    match reqwest::get(link).await {
        Ok(link_data) => match link_data.bytes().await {
            Ok(link_data_bytes) => {
                let file = File::create(file_path);

                match file {
                    Ok(mut file) => {
                        match std::io::copy(
                            &mut Cursor::new(link_data_bytes),
                            &mut file,
                        ) {
                            Ok(_) => {
                                return Ok(());
                            }

                            Err(err) => {
                                return Err(err.to_string());
                            }
                        }
                    }

                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            }

            Err(err) => {
                return Err(err.to_string());
            }
        },

        Err(err) => {
            return Err(err.to_string());
        }
    }
}