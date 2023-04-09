const ROOT: &str = ".rgit";
const OBJECTS: &str = "/objects";
const REFS: &str = "/refs";
const INDEX: &str = "/index.json";

pub struct Paths;

impl Paths {
    pub fn root() -> String {
        String::from(ROOT)
    }

    pub fn objects() -> String {
        Paths::root() + OBJECTS
    }

    pub fn refs() -> String {
        Paths::root() + REFS
    }

    pub fn index() -> String {
        Paths::root() + INDEX
    }

    pub fn dirs() -> Vec<String> {
        vec![Paths::objects(), Paths::refs()]
    }
}
