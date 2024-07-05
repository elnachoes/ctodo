use std::{
    fs::File, 
    io::{Read, Write}
};

use serde::{Serialize, Deserialize};

use crate::get_base_dir;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub editor : String,
}
impl Config {
    const FILE_NAME : &'static str = "config.json";
    
    pub fn load() -> Self {
        if let Ok(mut file) = File::options().read(true).open(Self::get_path()) {
            let mut contents = String::default();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap()
        } else {
            Self::reset();
            Self::default()
        }
    }

    pub fn reset() {
        let mut f = File::options().create(true).write(true).open(Self::get_path()).unwrap();
        f.write_all(serde_json::to_string(&Self::default()).unwrap().as_bytes()).unwrap();
    }

    pub fn get_path() -> String {
        format!("{}/{}", get_base_dir(), Self::FILE_NAME)
    }
}
impl Default for Config {
    fn default() -> Self {
        Self { editor: "code".to_string() }
    }
}