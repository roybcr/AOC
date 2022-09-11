use std::{env, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn read_file(path: &str) -> String {
    let mut filepath: PathBuf = env::current_dir().unwrap();
    if let Some(parent) = filepath.parent() {
        if let Some(parent_str) = parent.to_str() {
            let p = format!("{}/inputs/{}", parent_str, path);
            filepath.push(p);
        }
    }

    println!("{:#?}", filepath.clone());

    fs::read_to_string(filepath).expect("The provided path doesn't exist.")
}
