use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write, Read};
use serde::{Serialize, Deserialize};

pub struct FileRepository<T> {
    file_path: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize + for<'a> Deserialize<'a>> FileRepository<T> {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn save(&self, records: Vec<T>) -> Result<(), String> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .map_err(|e| e.to_string())?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &records)
            .map_err(|e| e.to_string())
    }

    pub fn load(&self) -> Result<Vec<T>, String> {
        let file = File::open(&self.file_path)
            .map_err(|e| e.to_string())?;

        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
            .map_err(|e| e.to_string())
    }
} 