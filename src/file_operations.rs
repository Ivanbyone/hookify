use std::{fs::File, io::Write, path::Path};

pub struct FileOperations;

impl FileOperations {
    pub fn new() -> Self {
        FileOperations
    }
    pub fn create_file<P>(&self, path: P, configuration: &str) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        // let data = String::from(configuration);
        File::create(path)?.write_all(configuration.as_bytes())?;
        Ok(())
    }
}
