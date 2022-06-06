use std::collections::HashSet;

pub struct Room {
    name: String,
    devices: HashSet<String>,
}

impl Room {
    pub fn new(name: String, devices: HashSet<String>) -> Self {
        Room { name, devices }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_device(&self, name: &str) -> Option<&str> {
        for device in &self.devices {
            if device.as_str() == name {
                return Some(device.as_str());
            };
        }

        None
    }
    pub fn get_devices(&self) -> impl Iterator<Item = &String> {
        self.devices.iter()
    }

    pub fn insert_device(&mut self, device_name: String) -> bool {
        self.devices.insert(device_name)
    }

    pub fn remove_device(&mut self, device_name: &str) -> bool {
        self.devices.remove(device_name)
    }
}
