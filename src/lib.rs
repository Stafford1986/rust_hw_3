use std::collections::HashSet;
mod storage_example;

pub struct Room {
    name: String,
    devices: HashSet<String>,
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
    pub fn get_devices(&self) -> impl Iterator<Item = &String> {
        self.devices.iter()
    }
}

pub trait RoomsStorage<'a> {
    type IterType: Iterator<Item = (&'a String, &'a Room)>;
    fn list_rooms(&self) -> Self::IterType;
    fn add_room(&mut self, room_name: &str, room: Room) -> Result<&Room, String>;
    fn get_room(&self, name: &str) -> Result<&Room, String>;
    fn delete_room(&mut self, name: &str) -> Result<(), String>;
    fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<(), String>;
    fn delete_device(&mut self, room_name: &str, device_name: &str) -> Result<(), String>;
}

pub struct Home<S>
where
    for<'a> S: RoomsStorage<'a>,
{
    name: String,
    rooms_storage: S,
}

pub trait ObjectReporter {
    fn get_device_state(&self, room: &str, device: &str) -> Result<&str, String>;
}

impl<S> Home<S>
where
    for <'a> S: RoomsStorage<'a>,
{
    pub fn new(name: String, rooms_storage: S) -> Self {
        Self {
            name,
            rooms_storage,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn list_rooms(&self) -> <S as RoomsStorage<'_>>::IterType {
        self.rooms_storage.list_rooms()
    }

    pub fn add_room(&mut self, room_name: &str, room: Room) -> Result<&Room, String> {
        self.rooms_storage.add_room(room_name, room)
    }

    pub fn delete_room(&mut self, name: &str) -> Result<(), String> {
        self.rooms_storage.delete_room(name)
    }

    pub fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<(), String> {
        self.rooms_storage.add_device(room_name, device_name)
    }

    pub fn delete_divece(&mut self, room_name: &str, device_name: &str) -> Result<(), String> {
        self.rooms_storage.delete_device(room_name, device_name)
    }

    pub fn get_report<R: ObjectReporter>(&self, reporter: R) -> String {
        let mut report = String::new();
        report.push_str(format!("Report for: {}", self.name()).as_str());

        let rooms_iter = self.rooms_storage.list_rooms();
        for (room_name, room) in rooms_iter {
            let devices = room.get_devices();
            for device in devices {
                let state = reporter.get_device_state(&room.name, device.as_str());
                match state {
                    Ok(state) => report.push_str(
                        format!(
                            "Room {}, has device {} with state - {}",
                            room_name, device, state,
                        )
                        .as_str(),
                    ),
                    Err(err) => {
                        print!("failed get device state: {}", err)
                    }
                }
            }
        }
        report
    }
}
