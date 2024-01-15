use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct CsvWriter {
    title: Option<String>,
}

impl CsvWriter {
    pub fn new(title: Option<String>) -> Self {
        Self { title }
    }

    pub fn save_to_file<P>(&self, file_path: P, lines_to_file: String) -> std::io::Result<File>
    where
        P: AsRef<Path>,
    {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open(&file_path)?;

        let lines_to_file = if let Some(title) = &self.title {
            format!("{title}\n{lines_to_file}")
        } else {
            lines_to_file
        };

        BufWriter::new(&file).write_all(lines_to_file.as_bytes())?;
        Ok(file)
    }
}
