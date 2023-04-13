use crate::{objects, storing::Storable, DirBuilder};

pub enum CommandReturnType {
    Storable(Box<dyn Storable>),
    NonStorable,
}

pub struct Commands;

impl Commands {
    pub fn init() -> CommandReturnType {
        CommandReturnType::Storable(Box::new(DirBuilder))
    }

    pub fn add(path: String) -> CommandReturnType {
        let mut index = objects::Index::new_from_index_file();
        index.add(path);
        CommandReturnType::Storable(Box::new(index))
    }

    pub fn status() -> CommandReturnType {
        let index = objects::Index::new_from_index_file();
        index.status();

        CommandReturnType::NonStorable
    }
}

pub struct DevCommands;

impl DevCommands {
    pub fn clean() -> CommandReturnType {
        CommandReturnType::Storable(Box::new(DirBuilder::clean()))
    }
}
