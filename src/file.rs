use directories::ProjectDirs;

use crate::error::Error;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

pub fn cookie_path() -> Result<(), Error> {
    let cookie_path = get_cookie_path()?;

    if !cookie_path.exists() {
        return Err(Error::CookieNotFound);
    }

    println!("{}", cookie_path.to_string_lossy());
    Ok(())
}

pub fn get_cookie_path() -> Result<std::path::PathBuf, Error> {
    if let Some(project_dir) = ProjectDirs::from("Recursion", "tool", "rlr") {
        let config_dir = project_dir.config_dir();
        Ok(config_dir.join("cookie.jar"))
    } else {
        Err(Error::CookiePathUnvaliable)
    }
}

pub fn save_to_file<P: AsRef<Path>>(path: &P, contents: &str) -> io::Result<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_file_name(file_path: &str) -> Result<&str, Error> {
    Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(Error::Internal(
            "Failed to get file name in get_file_name".to_string(),
        ))
}
