use std::process::Command;
use sysinfo::Disks;

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
    pub fn device_type(&self) -> DeviceType {
        self.device_type.clone()
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

pub fn get_all_devices() -> Vec<Device> {
    // USB devices
    // TODO: Add network mounts
    Disks::new_with_refreshed_list()
        .into_iter()
        .filter(|d| d.is_removable())
        .map(|d| Device {
            device_type: DeviceType::USB,
            label: d
                .mount_point()
                .file_name()
                .map_or(String::new(), |name| name.to_string_lossy().into_owned()),
            mountpoint: d.mount_point().display().to_string(),
            mounted: true,
        })
        .collect()
}
