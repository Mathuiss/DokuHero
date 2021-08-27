use std::env::current_dir;
use std::fs::File;
use std::path::Path;

pub fn locate_file(raw_input: &str) -> Result<File, std::io::Error> {
    if raw_input.starts_with('/') {
        return File::open(raw_input);
    } else {
        let cwd = current_dir()?;
        let cwd = cwd.to_str().unwrap();
        return File::open(format!("{}/{}", cwd, raw_input));
    }
}

pub fn dir_exists(raw_input: &str) -> Result<bool, std::io::Error> {
    if raw_input.starts_with('/') {
        return Ok(Path::new(raw_input).exists());
    } else {
        let cwd = current_dir()?;
        let cwd = cwd.to_str().unwrap();
        return Ok(Path::new(&format!("{}/{}", cwd, raw_input)).exists());
    }
}
