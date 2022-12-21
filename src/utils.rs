use std::path::PathBuf;
use std::io::BufRead;
use std::io;
use std::fs::File;

pub fn freadlines(path: PathBuf) -> Vec<String> {
    io::BufReader::new(
        File::open(
            path
        ).unwrap()
    ).lines()
    .map(|l| l.unwrap())
    .collect()
}
