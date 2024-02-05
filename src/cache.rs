#![allow(dead_code)]

#[cfg(feature = "cache")]
use std::io::{BufRead, Write};
use std::path::PathBuf;

const CACHE_LIMIT: usize = 50;

pub(crate) struct Cache {
    #[cfg(feature = "cache")]
    data: Vec<String>,
}

pub fn path() -> PathBuf {
    crate::config::dir().join("cache.txt")
}

#[cfg(feature = "cache")]
impl Cache {
    pub fn new() -> Cache {
        Cache {
            data: Vec::with_capacity(CACHE_LIMIT),
        }
    }

    pub fn from_file(path: &PathBuf) -> Result<Cache, std::io::Error> {
        if !path.exists() {
            return Ok(Cache::new());
        }

        let mut data = Vec::with_capacity(CACHE_LIMIT);

        let file = std::fs::File::open(path)?;

        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            data.push(line?);
        }

        Ok(Cache { data })
    }

    pub fn push(&mut self, value: String) {
        if self.data.len() >= CACHE_LIMIT {
            self.data.remove(0);
        }

        self.data.push(value);
    }

    pub fn contains(&self, value: &str) -> bool {
        self.data.contains(&value.to_string())
    }

    pub fn bust(&mut self) -> &mut Self {
        self.data.clear();

        self
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        if !path.exists() {
            std::fs::create_dir_all(path.parent().unwrap())?;
        }

        let mut file = std::fs::File::create(path)?;

        let data = self.data.join("\n");

        file.write_all(data.as_bytes())?;

        Ok(())
    }
}

#[cfg(not(feature = "cache"))]
impl Cache {
    pub fn new() -> Cache {
        Cache {}
    }

    pub fn from_file(_path: &PathBuf) -> Result<Cache, std::io::Error> {
        Ok(Cache::new())
    }

    pub fn push(&mut self, _value: String) {}

    pub fn contains(&self, _value: &str) -> bool {
        false
    }

    pub fn bust(&mut self) {}

    pub fn write(&self, _path: &PathBuf) -> Result<(), std::io::Error> {
        Ok(())
    }
}
