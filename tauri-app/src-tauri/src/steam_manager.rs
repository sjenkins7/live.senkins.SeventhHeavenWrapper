use steamlocate::SteamDir;
use std::{fs, path::{Path, PathBuf}};

use log::{debug, warn};


pub struct SteamManager {
    home: PathBuf
}

impl SteamManager {
    pub fn get_game_path(path: &Path, app_id: u32) -> PathBuf {
        if let Ok(steam_dir) = SteamDir::from_dir(path) {
            let (game, lib) = steam_dir
                .find_app(app_id)
                .expect("Couldn't locate FF7")
                .unwrap();
            lib.resolve_app_dir(&game)
        } else {
            panic!("APP_ID '{}' not found in Path: '{}'", app_id, path.display())
        }
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
            if d.join("steam/steamapps/libraryfolders.vdf").exists() {
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
            home.join(".var/app/com.valvesoftware.Steam/.steam"),
            // Steam Native
            home.join(".steam")
        ].to_vec()
    }
}