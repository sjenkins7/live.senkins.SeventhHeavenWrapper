use steamlocate::SteamDir;
use std::{fs, path::PathBuf};

use log::{debug, warn};


pub struct SteamManager {
    home: PathBuf
}

impl SteamManager {
    pub fn get_game_path(&self, app_id: u32) -> Option<PathBuf> {
        if let Ok(steam_dir) = SteamDir::from_dir(&self.home) {
            if let Ok(Some((game, lib))) = steam_dir.find_app(app_id) {
                return Some(lib.resolve_app_dir(&game))
            }
        }
        warn!("Couldn't detect FF7install path");
        None
    }
    
    pub fn can_read_path(path: &PathBuf) -> bool {
        fs::metadata(path).is_ok()
    }

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