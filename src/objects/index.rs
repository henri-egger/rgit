pub struct Index {
    entry_count: u32,
    entries: Vec<Entry>,
    sha1: String,
}

struct Entry {
    ctime: u64,
    mtime: u64,
    dev: u32,
    ino: u32,
    mode: u32,
    uid: u32,
    gid: u32,
    file_size: u32,
    sha1: String,
    file_path: String,
}
