use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseAsset {
    pub browser_download_url: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseResponse {
    pub url: String,
    pub assets: Vec<ReleaseAsset>,
    pub id: i32,
}

pub struct DownloadData {
    pub frs_exe: String,
    pub installer_exe: String,
    pub uninstaller_exe: String,
    pub version: i32,
}