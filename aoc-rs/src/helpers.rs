use std::{env, fs, path::PathBuf};

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn read_file(path: &str) -> String {
    let mut filepath: PathBuf = env::current_dir().unwrap();
    filepath.push(format!("{}/{}", "src", path));
    fs::read_to_string(filepath).expect("The provided path doesn't exist.")
}
