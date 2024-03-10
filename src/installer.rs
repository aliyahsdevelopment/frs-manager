use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ReleaseAsset {
    pub browser_download_url: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReleaseResponse {
    pub url: String,
    pub assets: Vec<ReleaseAsset>,
}

#[tokio::main]
async fn main() {
    async fn get_download_url() -> Result<String, String> {
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
                                        return Ok(asset.browser_download_url);
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

    let download_url = get_download_url().await;

    match download_url {
        Ok (download_url) => {
            println!("url: {}", download_url);
        }

        Err(err) => {
            println!("Error whilst fetching download url: {}", err)
        }
    }
}
