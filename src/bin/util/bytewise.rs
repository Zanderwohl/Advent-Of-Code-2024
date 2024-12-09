use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct ByteIterator {
    file: File,
    length: u64,
}

impl ByteIterator {
    pub fn new<P>(file_path: P) -> io::Result<Self>
    where P: AsRef<Path>,{
        let file = File::open(file_path)?;
        let metadata = file.metadata()?;
        let length = metadata.len();
        Ok(Self { file, length })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }
}

impl Iterator for ByteIterator {
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; 1];
        match self.file.read(&mut buffer) {
            Ok(0) => None, // End of file
            Ok(_) => Some(Ok(buffer[0])),
            Err(e) => Some(Err(e)), // Return the error
        }
    }
}
