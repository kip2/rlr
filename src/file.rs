use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

pub fn save_to_file(path: &str, contents: &str) -> io::Result<()> {
    let path = Path::new(path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
