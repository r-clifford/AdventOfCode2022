use std::collections;

enum Commands {
    CD,
    LS,
}
struct File {
    size: usize,
    members: Vec<File>,
    parent: &'static File,
}

struct FileSystem {
    root: File,
}
pub fn test7a() {
    
}