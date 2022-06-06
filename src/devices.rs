use crate::home::ObjectReporter;
use std::collections::HashMap;

pub trait DeviceItem {
    fn get_name(&self) -> &str;
    fn get_state(&self) -> String;
}

pub struct DeviceProvider<T: DeviceItem> {
    room_device_map: HashMap<String, Vec<T>>,
}

impl<T: DeviceItem> DeviceProvider<T> {
    pub fn new(device_map: HashMap<String, Vec<T>>) -> Self {
        DeviceProvider {
            room_device_map: device_map,
        }
    }
}

impl<T: DeviceItem> ObjectReporter for DeviceProvider<T> {
    fn get_device_state(&self, room: &str, device: &str) -> Result<String, String> {
        let devices = match self.room_device_map.get(room) {
            Some(dev) => dev,
            None => return Err(format!("room {} not found", room)),
        };

        let item = devices.iter().find(|v| v.get_name() == device);
        match item {
            Some(i) => Ok(i.get_state()),
            None => Err(format!("device {} not found", device)),
        }
    }
}
