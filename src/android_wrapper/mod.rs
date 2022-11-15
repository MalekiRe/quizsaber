use std::env;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};


pub fn get_config_file() -> Result<String> {
    let mut file = get_config_file_file()?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

pub fn save_config_file(file_contents: String) -> Result<()> {
    let mut config_file = get_config_file_file()?;
    config_file.write_all(file_contents.as_bytes())?;
    Ok(())
}

fn get_config_file_file() -> Result<File> {
    let mut working_dir = get_working_dir()?;
    working_dir.push(Path::new("config.config"));
    if !working_dir.exists() {
        File::create(working_dir.as_path())?;
    }
    let mut file = File::options().read(true).write(true).open(working_dir.as_path())?;
    Ok(file)
}

pub fn get_working_dir() -> Result<PathBuf> {
    if cfg!(target_os = "android") {
        let mut data: Option<&str> = None;
        #[cfg(target_os = "android")]
        {
            data = Some(ndk_glue::native_activity().external_data_path().to_str().context("yo")?)
        }
        let data = data.unwrap();
        Ok(PathBuf::from(data.to_string()))
    } else {
        Ok(env::current_dir()?)
    }
}