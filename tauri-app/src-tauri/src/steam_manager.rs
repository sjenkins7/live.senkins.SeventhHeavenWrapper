use std::path::PathBuf;

use log::{debug, warn};


pub struct SteamManager {
    home: PathBuf
}

impl SteamManager {
    pub fn new(home: PathBuf) -> SteamManager {
        SteamManager {
            home
        }
    }

    pub fn detect_steam_home() -> Option<PathBuf>{
        for d in Self::known_steam_directories() {
            debug!("Testing {} for libraryfolders.vdf", d.display());
            if d.join("steamapps/libraryfolders.vdf").exists() {
                return Some(d);
            }
        }
        warn!("Couldn't detect an install of Steam");
        None
    }

    fn known_steam_directories() -> Vec<PathBuf> {
        let home = home::home_dir().unwrap();
        [
            // Steam on Flatpak
            home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
            // Steam Native
            home.join(".local/share/Steam")
        ].to_vec()
    }
}