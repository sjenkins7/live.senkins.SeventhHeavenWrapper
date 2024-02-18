use std::{process::Command, fs::{File, self}, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

pub(crate) fn install_package(prefix: &PathBuf, package: &String) -> Result<(), String>{
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Unable to get current time").as_secs();
    let wait = Command::new("winetricks")
    .arg("-q")
    .arg(package)
    .env("WINEPREFIX", prefix)
    .stdout(installer_log(prefix, format!("{time}_winetricks_{package}_stdout.log")))
    .stderr(installer_log(prefix, format!("{time}_winetricks_{package}_stderr.log")))
    .spawn().expect("Failed to execute winetricks - should always be available").wait();

    let status = match wait {
        Ok(retval) => retval,
        Err(e) => return Err(e.to_string())
    };

    if ! status.success() {
        Err("Failed to install package".to_string())
    } else {
        Ok(())
    }
    
}

fn installer_log(prefix: &PathBuf, filename: String) -> File {
    let log_prefix = prefix.join("installer_logs");
    let log_name = log_prefix.join(filename);
    fs::create_dir_all(log_prefix).expect("failed to create log dir");
   File::create(log_name).expect("failed to open log")
}