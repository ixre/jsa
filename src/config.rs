use std::fs::File;
use std::io::{ErrorKind, Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            db_url: String::from(""),
        }
    }
}

/// Return path of config file
pub fn path() -> String {
    let mut file_path = super::CONF_PATH.lock().unwrap().clone();
    file_path.push_str("/config.toml");
    file_path
}

pub fn read_config(path: &str) -> Config {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                flush_defaults(path);
                File::open(path).unwrap()
            } else {
                panic!("Open {} except:{}", path.clone(), err);
            }
        }
    };
    let mut buf: String = String::new();
    match file.read_to_string(&mut buf) {
        Ok(s) => s,
        Err(err) => panic!("Read {} except:{}", path.clone(), err),
    };
    let dst: Result<Config, toml::de::Error> = toml::from_str(&buf);
    match dst {
        Ok(arr) => arr,
        Err(err) => panic!("Deserialize users except:{}", err),
    }
}

// If config.toml not exists, create the default config.
fn flush_defaults(path: &str) -> Config {
    let c = Config {
        db_url: "postgres://postgres:123456@localhost:5432/jsa".to_owned(),
    };
    save(path, &c);
    c
}

// Save user's settings to file
fn save(path: &str, u: &Config) {
    match toml::to_string_pretty(&u) {
        Ok(s) => match File::create(path) {
            Ok(mut fi) => {
                if let Err(err) = fi.write(s.as_bytes()) {
                    panic!("Save config file except: {}", err);
                }
            }
            Err(err) => panic!("Convert entity to string :{}", err),
        },
        Err(err) => panic!("Convert entity to string :{}", err),
    }
}
