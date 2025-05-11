use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

pub fn save_to_file<P: AsRef<Path>>(path: &P, contents: &str) -> io::Result<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn get_file_name(file_path: &str) -> &str {
    Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap()
}
