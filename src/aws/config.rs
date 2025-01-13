use anyhow::Result;
use std::{env, path::Path};

use ini::Ini;

fn file_path() -> String {
    env::var("AWS_CONFIG_FILE").unwrap_or_else(|_| {
        let home = dirs::home_dir().unwrap();
        let mut path = home.clone();
        path.push(".aws");
        path.push("config");
        path.to_str().unwrap().to_string()
    })
}

pub struct Config {
    pub file_path: String,
    pub ini: Ini,
}

impl Config {
    pub fn load_or_new() -> Result<Self> {
        let file_path = file_path();
        // the file does not exist
        if Path::new(&file_path).exists() {
            let ini = Ini::load_from_file(&file_path)?;
            Ok(Config { file_path, ini })
        } else {
            println!("{} not found, so creating a new one", &file_path);
            Ok(Config {
                file_path,
                ini: Ini::new(),
            })
        }
    }

    pub fn save(&self) -> Result<()> {
        self.ini.write_to_file(&self.file_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_file_path() {
        let file = file_path();
        assert_eq!(file, "/home/newgyu/.aws/config");
    }
}
