use std::fs::{DirEntry, ReadDir};
use std::path::Path;

pub struct RecursiveReadDir {
    stack: Vec<ReadDir>,
}

pub trait ReadDirExt {
    fn recursive(self) -> RecursiveReadDir;
}

impl ReadDirExt for ReadDir {
    fn recursive(self) -> RecursiveReadDir {
        RecursiveReadDir { stack: vec![self] }
    }
}

impl Iterator for RecursiveReadDir {
    type Item = std::io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(dir) = self.stack.last_mut() {
                if let Some(entry) = dir.next() {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(err) => return Some(Err(err)),
                    };
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(dir) = std::fs::read_dir(path) {
                            self.stack.push(dir);
                        }
                    }
                    return Some(Ok(entry));
                } else {
                    self.stack.pop();
                }
            } else {
                return None;
            }
        }
    }
}

pub fn list_dir_to_vec_recursive(dir: impl AsRef<Path>) -> std::io::Result<Vec<DirEntry>> {
    std::fs::read_dir(dir)?
        .recursive()
        .collect::<Result<Vec<_>, _>>()
}