use std::marker::PhantomData;

pub struct Room {
    name: String,
    devices: Vec<String>,
}

pub struct DeviceIterator<'a> {
    count: usize,
    devices: &'a Vec<String>,
}

impl<'a> Iterator for DeviceIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.devices.len() {
            let index = self.count;
            self.count += 1;
            Some(self.devices[index].as_str())
        } else {
            None
        }
    }
}

impl Room {
    pub fn get_device(&self, name: &str) -> Option<&str> {
        for device in &self.devices {
            if device.as_str() == name {
                return Some(device.as_str());
            };
        }
        
        None
    }
    pub fn get_devices(&self) -> DeviceIterator {
        DeviceIterator {
            count: 0,
            devices: &self.devices,
        }
    }
}

pub trait RoomsStorage<'a> {
    type ItemIterator: Iterator<Item = Room>;
    fn list_rooms(&self) -> Self::ItemIterator;
    fn add_room(&mut self, room: &str) -> Result<&Room, String>;
    fn get_room(&self, name: &str) -> Result<&Room, String>;
    fn delete_room(&mut self, name: &str) -> Result<(), String>;
    fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<&str, String>;
    fn delete_device(&mut self, room_name: &str, device_name: &str) -> Result<(), String>;
}

pub struct Home<'a, S>
where
    S: RoomsStorage<'a>,
{
    name: String,
    rooms_storage: S,
    phantom: PhantomData<&'a S>,
}

pub trait ObjectReporter {
    fn get_device_state(&self, room: &str, device: &str) -> &str;
}

impl<'a, S> Home<'a, S>
where
    S: RoomsStorage<'a>,
{
    pub fn new(name: String, rooms_storage: S) -> Self {
        Self {
            name,
            rooms_storage,
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn list_rooms(&self) -> S::ItemIterator {
        self.rooms_storage.list_rooms()
    }

    pub fn add_room(&mut self, room: &str) -> Result<&Room, String> {
        self.rooms_storage.add_room(room)
    }

    pub fn delete_room(&mut self, name: &str) -> Result<(), String> {
        self.rooms_storage.delete_room(name)
    }

    pub fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<&str, String> {
        self.rooms_storage.add_device(room_name, device_name)
    }

    pub fn delete_divece(&mut self, room_name: &str, device_name: &str) -> Result<(), String> {
        self.rooms_storage.delete_device(room_name, device_name)
    }

    pub fn get_report<R: ObjectReporter>(&self, reporter: R) -> String {
        let mut report = String::new();
        report.push_str(format!("Report for: {}", self.name()).as_str());

        let rooms_iter = self.rooms_storage.list_rooms();
        for room in rooms_iter {
            let devices = room.get_devices();
            for device in devices {
                let state = reporter.get_device_state(&room.name, device);
                report.push_str(
                    format!(
                        "Room {}, has device {} with state - {}",
                        &room.name, device, state,
                    )
                    .as_str(),
                )
            }
        }
        report
    }
}
