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
        let source_dir = self.get_c_path(source_dir).display().to_string();
        let drive_path = self.get_drive_path(drive_letter);
        if drive_path.exists() || drive_path.is_symlink() {
            if drive_path.is_symlink() {
                let existing_link = std::fs::read_link(&drive_path)?;
                let existing_link = existing_link.to_str()
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Failure to read existing link!"))?;

                if existing_link == source_dir {
                    return Ok(info!("{} is already mounted in {}!", source_dir, drive_letter))
                }
            }
            warn!("Something else is mounted in drive {}! Unmounting...", drive_letter);
            std::fs::remove_file(&drive_path)?;
        }

        os::unix::fs::symlink(&source_dir, drive_path).unwrap();
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
