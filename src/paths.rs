const ROOT: &str = ".rgit";
const DIRS: [&str; 2] = ["/objects", "/refs"];
const INDEX: &str = "/index.json";

pub struct Paths;

impl Paths {
    pub fn root() -> String {
        String::from(ROOT)
    }

    pub fn dirs() -> Vec<String> {
        let mut dirs = Vec::new();

        for dir in DIRS {
            dirs.push(String::from(ROOT) + dir);
        }

        dirs
    }

    pub fn index() -> String {
        String::from(ROOT) + &INDEX
    }
}
