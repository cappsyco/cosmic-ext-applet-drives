use notify_rust::Notification;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

#[derive(Clone)]
pub enum DeviceType {
    USB,
    _Network,
}

#[derive(Clone)]
pub struct Device {
    device_type: DeviceType,
    label: String,
    mountpoint: String,
    mounted: bool,
}

impl Device {
    #[must_use]
    pub fn device_type(&self) -> DeviceType {
        self.device_type.clone()
    }
    #[must_use]
    pub fn label(&self) -> String {
        self.label.clone()
    }
    #[must_use]
    pub fn mountpoint(&self) -> String {
        self.mountpoint.clone()
    }
    #[must_use]
    pub fn mounted(&self) -> bool {
        self.mounted
    }
}

pub fn get_all_devices() -> std::io::Result<Vec<Device>> {
    let mut devices = vec![];

    // read in all active mounts
    let file = File::open("/proc/mounts")?;
    for line in BufReader::new(file).lines() {
        let Ok(line) = line else { continue };

        // break up line into block device and mount point
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        let mountpoint = line_parts[1].replace("\\040", " ");

        // simple and dirty check to see if the drive is removable media
        // we want to be listing. not all are properly flagged as removable
        // and this also removes dupes mouinted on /run/host
        if mountpoint.starts_with("/run/media/") {
            // break up mountpoint to get the device label
            let mountpoint_parts: Vec<&str> = mountpoint.split('/').collect();
            let label = mountpoint_parts[mountpoint_parts.len() - 1];
            devices.push(Device {
                device_type: DeviceType::USB,
                label: label.to_owned(),
                mountpoint: mountpoint.clone(),
                mounted: true,
            });
        }
    }
    Ok(devices)
}

pub fn run_command(cmd: &str, mountpoint: &str) {
    match if is_flatpak() {
        Command::new("flatpak-spawn")
            .arg("--host")
            .arg(cmd)
            .arg(mountpoint)
            .status()
    } else {
        Command::new(cmd).arg(mountpoint).status()
    } {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}

pub fn _send_notification(title: &str, desc: &str) {
    match Notification::new()
        .summary(title)
        .body(desc)
        .icon("media-eject-symbolic")
        .show()
    {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}

#[cfg(feature = "flatpak")]
fn is_flatpak() -> bool {
    true
}

#[cfg(not(feature = "flatpak"))]
fn is_flatpak() -> bool {
    false
}
