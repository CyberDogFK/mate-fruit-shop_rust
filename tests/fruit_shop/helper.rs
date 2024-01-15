use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

#[derive(Debug)]
pub struct TestFile {
    file: File,
    pub name: String,
}

impl TestFile {
    pub fn create_with_random_name() -> std::io::Result<TestFile> {
        let name = Uuid::new_v4().to_string();
        Ok(Self {
            file: OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(false)
                .open(name.as_str())?,
            name,
        })
    }

    pub fn from_file(name: String) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&name)
            .unwrap();
        Self { file, name }
    }

    pub fn read_from_file(&self) -> String {
        let mut buffer = String::new();
        BufReader::new(self.deref())
            .read_to_string(&mut buffer)
            .unwrap();
        buffer
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        std::fs::remove_file(&self.name).expect("Can't remove file!")
    }
}

impl Deref for TestFile {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for TestFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}

impl AsRef<File> for TestFile {
    fn as_ref(&self) -> &File {
        &self.file
    }
}
