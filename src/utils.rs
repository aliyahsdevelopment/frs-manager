use crate::models::{DownloadData, ReleaseResponse};
use std::path::Path;

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

#[cfg(target_os = "windows")]
pub fn remove_env_var(download_path: &Path) {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    match std::env::var("PATH") {
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
            } else {
                println!("FRS already doesnt exist in env path");
            }
        }

        Err(err) => {
            println!("Couldnt fetch windows env variables: {}", err);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn remove_env_var(_download_path: &Path) {

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