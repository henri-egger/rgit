use std::{convert::AsRef, fs, path};

pub struct IgnoreFilter {
    ignore_paths: Vec<String>,
}

impl IgnoreFilter {
    /// Creates a new ignore filter loaded with the paths mentioned in .gitignore
    pub fn new(ignore_path: impl AsRef<path::Path>) -> IgnoreFilter {
        let ignore_files = fs::read_to_string(ignore_path).unwrap() + "\n.git\n.rgit";
        let ignore_paths = ignore_files.lines().map(|x| x.to_string()).collect();

        IgnoreFilter { ignore_paths }
    }

    pub fn is_valid(&self, path: &str) -> bool {
        !self
            .ignore_paths
            .iter()
            .any(|x| path.starts_with(&(x.to_owned() + "/")) || path.eq(x))
    }
}
