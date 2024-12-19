use super::Error;

use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::structs::Platform;

pub struct UsbInfo {
    //TODO make baseline a HashSet<PathBuf>
    pub baseline: Vec<PathBuf>, // Store baseline as a list of PathBuf objects
}

impl UsbInfo {
    //TODO: create two new methods:
    //Parse the command output.stdout :: PARSE COMMAND
    //ls on the given path :: LS COMMAND
    pub fn new(platform: &Platform) -> Result<Self, Error> {
        Ok(UsbInfo{
            baseline: UsbInfo::query_devices(&platform)?,
        })
    }
    // Method to query devices and return a vector of PathBufs
    pub fn query_devices(os: &Platform) -> Result<Vec<PathBuf>, Error> {
        let mut device_paths: Vec<PathBuf> = Vec::new();
        match os {
            Platform::Windows => {
                //TODO test on Windows
                // On Windows, use `wmic` to list logical disks (drives)
                match Command::new("wmic")
                    .args(&["logicaldisk", "get", "name"])
                    .output()//Use question mark remove match
                {
                    Ok(output) => {
                        //PARSE COMMAND ONE
                        let result = String::from_utf8_lossy(&output.stdout);
                        for line in result.lines().filter(|l| !l.is_empty()) {
                            device_paths.push(PathBuf::from(line.trim()));
                        }
                    }
                    Err(e) => {
                        return Err(Error::Exited("Failed to query devices on windows: {}".to_string()))
                    }
                }
            }
            Platform::Linux => {
                // On Linux, list the contents of `/media/$USER` or `/mnt`
                if let Some(user) = env::var_os("USER") {
                    let media_path = PathBuf::from(format!("/media/{}", user.to_string_lossy()));
                    //LS COMMAND ONE
                    match Command::new("ls")
                        .arg(&media_path)
                        .output()//Use question mark remove match
                    {
                        Ok(output) => {
                            //PARSE COMMAND TWO
                            let result = String::from_utf8_lossy(&output.stdout);
                            for line in result.lines().filter(|l| !l.is_empty()) {
                                device_paths.push(media_path.join(line.trim()));
                            }
                        }
                        Err(_) => {
                            // If `/media/$USER` fails, fallback to `/mnt`
                            let mnt_path = PathBuf::from("/mnt");
                            //LS COMMAND TWO
                            match Command::new("ls")
                                .arg(&mnt_path)
                                .output()//Use question mark remove match
                            {
                                Ok(output) => {
                                    //PARSE COMMAND Three
                                    let result = String::from_utf8_lossy(&output.stdout);
                                    for line in result.lines().filter(|l| !l.is_empty()) {
                                        device_paths.push(mnt_path.join(line.trim()));
                                    }
                                }
                                Err(e) => {
                                    return Err(Error::Exited("Failed to query devices on Linux: {}".to_string()))
                                }
                            }
                        }
                    }
                }
            }
            Platform::Mac => {
                // On macOS, list the contents of `/Volumes`
                let volumes_path = PathBuf::from("/Volumes");
                //LS COMMAND THREE
                //TODO test on mac
                match Command::new("ls")
                    .arg(&volumes_path)
                    .output()//Use question mark remove match
                {
                    Ok(output) => {
                        //PARSE COMMAND FOUR
                        let result = String::from_utf8_lossy(&output.stdout);
                        for line in result.lines().filter(|l| !l.is_empty()) {
                            device_paths.push(volumes_path.join(line.trim()));
                        }
                    }
                    Err(e) => {
                        return Err(Error::Exited("Failed to query devices on MacOS: {}".to_string()))
                    }
                }
            }
            _ => {
                println!("Unsupported operating system");
            }
        }
        Ok(device_paths)
    }

    // Method to find the device path by comparing baseline and new snapshot
    pub fn detect_new_device_path(&mut self, os: &Platform) -> Result<Option<PathBuf>, Error> {
        // Convert baseline into a HashSet of PathBufs for comparison
        let baseline_hash: HashSet<PathBuf> = self.baseline.clone().into_iter().collect();
        // Obtain a new snapshot of the device list
        let latest_hash: HashSet<PathBuf> = UsbInfo::query_devices(os)?.into_iter().collect();
        // Find the difference between the two sets
        let differences: Vec<&PathBuf> = latest_hash.difference(&baseline_hash).collect();
        //Error if more than one unique path is found
        if differences.len() > 1 {
            return Err(Error::Exited("More than one target USB device found".to_string()));
        }

        //if exactly 1 device is found, return the new device path
        if let Some(&device) = differences.first() {
            let new_device = device.clone().to_path_buf();
            return Ok(Some(new_device));
        }
        // If no new device was found, return None
        Ok(None)
    }
}
