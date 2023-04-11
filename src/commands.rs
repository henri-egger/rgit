use crate::objects;
use std::{fmt, path};

pub fn add(path: impl AsRef<path::Path> + fmt::Display) -> objects::Index {
    let mut index = objects::Index::from_index_file();
    index.add(path);
    index
}
