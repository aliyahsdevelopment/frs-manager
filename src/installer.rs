use std::io::{prelude::*, Cursor};
use std::process::Command;
use std::{fs::File, path::Path};
use serde::{Deserialize, Serialize};
use tokio::fs::create_dir;

#[derive(Serialize, Deserialize, Debug)]
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

#[tokio::main]
async fn main() {
    async fn get_download_url() -> Result<(String, i32), String> {
        let reqwest_client = reqwest::Client::builder()
            .user_agent("FRS-Manager")
            .build();

        match reqwest_client {
            Ok(reqwest_client) => {
                match reqwest_client.get("https://api.github.com/repos/Z3rio/frs-manager/releases/latest").send().await {
                    Ok(resp) => {
                        match resp.json::<ReleaseResponse>().await {
                            Ok(json) => {
                                let frs_exe_asset = json.assets.into_iter().find(|a| a.name == "frs.exe");

                                match frs_exe_asset {
                                    Some(asset) => {
                                        return Ok((asset.browser_download_url, json.id));
                                    }

                                    None => {
                                        return Err(String::from("Could not find asset in assets list"));
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
                }
            }

            Err(err) => {
                return Err(err.to_string());
            }
        }
    }

    fn get_download_path() -> Result<String, String> {
        match std::env::var("ProgramFiles") {
            Ok(program_files_path) => {
                return Ok(Path::new(program_files_path.as_str()).join("FRS_Manager").display().to_string());
            }

            Err(err) => {
                return Err(err.to_string());
            }
        };
    }

    let download_data = get_download_url().await;
    let raw_download_path = get_download_path();

    match download_data {
        Ok(download_data) => {
            match raw_download_path {
                Ok(raw_download_path) => {
                    println!("url: {}", download_data.0);
                    println!("id: {}", download_data.1);
                    println!("path: {}", raw_download_path);

                    let download_path = Path::new(raw_download_path.as_str());

                    if download_path.exists() == false {
                        create_dir(download_path).await.expect("Couldnt create download path");
                        println!("Created folder for frs-manager");
                    } else {
                        println!("Folder already exists");
                    }

                    let version_id_path = download_path.join("version_id");
                    if version_id_path.exists() == false {
                        let version_file = File::create(version_id_path);
                        match version_file {
                            Ok (mut version_file) => {
                                match version_file.write(download_data.1.to_string().as_bytes()) {
                                    Ok (_) => {
                                        println!("Wrote to version file");
                                    }

                                    Err (err) => {
                                        println!("Error occured whilst writing to version file: {}", err);
                                    }
                                }
                            }

                            Err (err) => {
                                println!("Error occured whilst creating version file: {}", err);
                            }
                        }
                    }

                    let frs_path = download_path.join("frs.exe");
                    if frs_path.exists() == false {
                        match reqwest::get(download_data.0).await {
                            Ok(frs_exe_data) => {
                                match frs_exe_data.bytes().await {
                                    Ok (frs_exe_data_bytes) => {
                                        let frs_file = File::create(frs_path);

                                        match frs_file {
                                            Ok (mut frs_file) => {
                                                match std::io::copy(&mut Cursor::new(frs_exe_data_bytes), &mut frs_file) {
                                                    Ok (_) => {
                                                        println!("Wrote to frs exe file");
                                                    }

                                                    Err (err) => {
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
                                }
                            }

                            Err(err) => {
                                println!("Error whilst fetching frs data: {}", err);
                            }
                        }
                    }
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

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
