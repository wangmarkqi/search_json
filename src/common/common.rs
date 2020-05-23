use walkdir::WalkDir;
use std::ffi::OsString;
use std::sync::mpsc;
use std::fs::{read_to_string, File};
use std::io::prelude::*;

// std::fs::create_dir_all()
// std::path::Path::exists()
// std::path::Path::new().extension()
pub fn get_all_local_path(root: String) -> Vec<String> {
    let mut v = vec![];
    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        let p = entry.path();
        if let Ok(pp) = OsString::from(p).into_string() {
            v.push(pp.clone());
        }
    }
    v
}


pub fn read_file_as_txt(file: &str) -> String {
    let res = read_to_string(file);
    if let Ok(s) = res {
        return s;
    }
    "".to_owned()
}
