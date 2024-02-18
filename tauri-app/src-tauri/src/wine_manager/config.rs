use std::{fs::File, io::ErrorKind, path::PathBuf};

use log::info;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct WineConfig {
    version: u32,
    installed_packages: Vec<String>,
}

impl WineConfig {
    fn config_filename(prefix: &PathBuf ) -> PathBuf {
        prefix.join("wrapper-config.yaml")
    }

    pub fn fetch(prefix: &PathBuf) -> WineConfig {
        let filename = Self::config_filename(prefix);
        let data: WineConfig = match File::open(&filename) {
            Ok(file) => match serde_yaml::from_reader(file) {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("There was an error parsing the YAML file {}", err);
                    std::process::exit(1);
                }
            },
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    info!("No config exists - assuming first time launch");
                    WineConfig::new()
                }
                other_error => {
                    panic!(
                        "Error opening config: {}: {:?}",
                        filename.display(),
                        other_error
                    );
                }
            },
        };
    
        data
    }
    fn new() -> WineConfig {
        WineConfig {
            version: 1,
            installed_packages: Vec::new()
        }
    }

    pub fn is_installed(&self, package: &String) -> bool {
        self.installed_packages.contains(package)
    }

    pub fn package_installed(&mut self, package: &String) {
        self.installed_packages.push(package.to_owned());
    }

    pub fn save(&self, prefix: &PathBuf) {
        // FIXME - better handling of failures
        let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(Self::config_filename(prefix))
        .expect("Couldn't open file");
        serde_yaml::to_writer(f, &self).unwrap();
    }
}