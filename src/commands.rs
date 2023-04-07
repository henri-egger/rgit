use crate::Paths;
use std::{fs, path};

pub fn init() {
    if path::Path::new(&Paths::root()).exists() == true {
        println!("Working directory is already a repository");
        return;
    }

    println!("Initlizing repository...");

    for dir in Paths::dirs() {
        fs::create_dir_all(&dir).expect(&format!("Failed to create directory {}", dir));
    }

    fs::File::create(Paths::index())
        .expect(&format!("Failed to create directory {}", Paths::index()));
}
