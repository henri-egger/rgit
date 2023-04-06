use std::{fs, path};

const ROOT: &str = ".rgit";
const DIRS: [&str; 2] = ["/objects", "/refs"];

pub fn init() {
    if path::Path::new(ROOT).exists() == true {
        println!("Working directory is already a repository");
        return;
    }

    println!("Initlizing repository...");

    for dir in DIRS {
        fs::create_dir_all(&format!("{}{}", ROOT, dir))
            .expect(&format!("Failed to create directory {}", dir));
    }
}
