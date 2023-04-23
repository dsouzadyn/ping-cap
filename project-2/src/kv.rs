#![allow(unused)]
use std::collections::HashMap;
use std::io::Result;
use std::path::PathBuf;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        eprintln!("unimplemented");
        Ok(())
    }
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        eprintln!("unimplemented");
        Ok(Some("".to_string()))
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        eprintln!("unimplemented");
        Ok(())
    }
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        eprintln!("unimplemented");
        Ok(KvStore {
            map: HashMap::new(),
        })
    }
}
