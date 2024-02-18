use std::path::PathBuf;

use log::{debug, warn};


pub struct SteamManager {
    home: PathBuf
}

impl SteamManager {
    pub fn new(home: PathBuf) -> SteamManager {
        SteamManager {
            home: home
        }
    }

    pub fn detect_steam_home() -> Option<PathBuf>{
        for d in Self::known_steam_directories() {
            debug!("Testing {} for libraryfolders.vdf", d.display());
            if d.join("steam/steamapps/libraryfolders.vdf").exists() {
                return Some(d);
            }
        }
        warn!("Couldn't detect an install of Steam");
        return None;
    }

    fn known_steam_directories() -> Vec<PathBuf> {
        let home = home::home_dir().unwrap();
        return [
            // Steam on Flatpak
            home.join(".var/app/com.valvesoftware.Steam/.steam"),
            // Steam Native
            home.join(".steam")
        ].to_vec()
    }
}