#[cfg(target_os = "windows")]
pub fn get_download_path() -> Result<String, String> {
    use std::path::Path;
    
    match std::env::var("ProgramFiles") {
        Ok(program_files_path) => {
            return Ok(Path::new(program_files_path.as_str())
                .join("FRS_Manager")
                .display()
                .to_string());
        }

        Err(err) => {
            return Err(err.to_string());
        }
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