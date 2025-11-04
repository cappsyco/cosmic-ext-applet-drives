use notify_rust::Notification;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

#[derive(Clone)]
pub enum DeviceType {
    USB,
    _Network,
}

#[derive(Clone)]
pub struct Device {
    device_type: DeviceType,
    block: String,
    label: String,
    mountpoint: String,
    mounted: bool,
}
impl Device {
    pub fn device_type(&self) -> DeviceType {
        self.device_type.clone()
    }
    pub fn block(&self) -> String {
        self.block.clone()
    }
    pub fn label(&self) -> String {
        self.label.clone()
    }
    pub fn mountpoint(&self) -> String {
        self.mountpoint.clone()
    }
    pub fn mounted(&self) -> bool {
        self.mounted
    }

    pub fn _mount(&self) {}
    pub fn _unmount(&self) {}
    pub fn _open(&self) {}
}

pub fn _get_all_devices_old() -> Vec<Device> {
    // USB devices
    // TODO: Add network mounts
    Disks::new_with_refreshed_list()
        .into_iter()
        .filter(|d| d.is_removable())
        .map(|d| Device {
            device_type: DeviceType::USB,
            block: String::new(),
            label: d
                .mount_point()
                .file_name()
                .map_or(String::new(), |name| name.to_string_lossy().into_owned()),
            mountpoint: d.mount_point().display().to_string(),
            mounted: true,
        })
        .collect()
}

pub fn get_all_devices() -> std::io::Result<Vec<Device>> {
    let mut devices = vec![];

    // read in all active mounts
    let file = File::open("/proc/mounts")?;
    for line in BufReader::new(file).lines() {
        let line = line?;

        // break up line into block device and mount point
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        let device = line_parts[0];
        let mountpoint = line_parts[1];

        // exclude /run/host/ mounts to avoid duplicates
        if !mountpoint.starts_with("/run/host/") && !mountpoint.is_empty() {
            // check that the device is removable
            if let Some(block) = device.strip_prefix("/dev/") {
                if is_removable(&block) {
                    // break up mountpoint to get the device label
                    let mountpoint_parts: Vec<&str> = mountpoint.split("/").collect();
                    let label = mountpoint_parts[mountpoint_parts.len() - 1];
                    devices.push(Device {
                        device_type: DeviceType::USB,
                        block: block.to_owned(),
                        label: label.to_owned(),
                        mountpoint: mountpoint.to_owned(),
                        mounted: true,
                    });
                }
            }
        }
    }
    Ok(devices)
}

fn is_removable(block: &str) -> bool {
    let removable_path = format!(
        "/sys/block/{}/removable",
        block.trim_end_matches(|c: char| c.is_ascii_digit())
    );
    if Path::new(&removable_path).exists() {
        if let Ok(contents) = fs::read_to_string(&removable_path) {
            if contents.trim() == "1" {
                return true;
            }
        }
    }
    false
}

pub fn run_command(cmd: &str, mountpoint: &str) {
    let result = if is_flatpak() {
        Command::new("flatpak-spawn")
            .arg("--host")
            .arg(cmd)
            .arg(mountpoint)
            .status()
    } else {
        Command::new(cmd).arg(mountpoint).status()
    };

    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {}", err)
        }
    }
}

pub fn _send_notification(title: &str, desc: &str) {
    Notification::new()
        .summary(title)
        .body(desc)
        .icon("media-eject-symbolic")
        .show()
        .unwrap();
}

#[cfg(feature = "flatpak")]
fn is_flatpak() -> bool {
    true
}

#[cfg(not(feature = "flatpak"))]
fn is_flatpak() -> bool {
    false
}
