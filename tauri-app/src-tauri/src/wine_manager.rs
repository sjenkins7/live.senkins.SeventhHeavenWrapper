use std::{env, os, path::PathBuf, process::Command, io};
use log::{warn, info};

mod winetricks;
mod config;

pub struct WineManager {
    prefix: PathBuf
}

impl WineManager {
    pub fn new() -> WineManager {
        let manager = WineManager {
            prefix: Self::get_prefix()
        };
        manager.init();
        manager
    }

    pub fn install_package(&mut self, package: &String) -> Result<(), String>{
        //FIXME ask winetricks
        // if self.config.is_installed(package) {
        //     debug!("Skipping install of [{}] - already installed", package);
        //     Ok(())
        // } else {
            winetricks::install_package(&self.prefix, package)
        // }
    }

    fn get_prefix() -> PathBuf {
        let home_dir: PathBuf;
        match env::var("WINEPREFIX") {
            Ok(val) => home_dir = PathBuf::from(val),
            Err(_e) => {
                warn!("No WINEPREFIX set - using default");
                match home::home_dir() {
                    Some(path) => home_dir = path.join(".wine"),
                    None => panic!("Cannot get home dir"),
                }
            }
        }
    
        home_dir
    }

    fn init(&self) {
        Command::new("wineboot")
        .arg("--init")
        .env("WINEPREFIX", &self.prefix)
        // Silence mono setup etc.
        .env("WINEDLLOVERRIDES", "mscoree=d,mshtml=d")
        //FIXME - return failure and bring up error screen.
        .spawn().expect("Failed to init prefix").wait().expect("Failed to init prefix");
    }

    pub fn launch_exe(&self, absolute_path: &str, vars: &Vec<(&str, &str)>, args: &Vec<&str>) -> std::io::Result<()> {
        let display = env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string());
        let mut command = Command::new("wine");
        command.arg(absolute_path);

        for (var, value) in vars {
            command.env(var, value);
        }
        for arg in args {
            command.arg(arg);
        }

        command
            .env("WINEPREFIX", &self.prefix)
            .env("DISPLAY", display)
            .spawn()?
            .wait()?;

        Ok(info!("Launched {absolute_path}"))
    }

    pub(crate) fn load_cd(&self, source_dir: &str, drive_letter: &str) -> io::Result<()> {
        let drive_path = self.get_drive_path(drive_letter);
        if !drive_path.exists() {
            os::unix::fs::symlink(format!("../drive_c/{}", source_dir), drive_path).unwrap();
        }

        // Kill wine so it actually loads our CD
        Command::new("wineserver")
            .arg("-w")
            .spawn()?
            .wait()?;

        Ok(info!("Loaded {} as drive {}:!", source_dir, drive_letter))
    }

    pub fn get_c_path(&self, source_dir: &str) -> PathBuf {
        self.prefix.join("drive_c").join(source_dir)
    }
    fn get_drive_path(&self, drive_letter: &str) -> PathBuf {
        self.prefix.join("dosdevices").join(format!("{}:", drive_letter))
    }
    
}
