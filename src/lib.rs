use std::{collections::HashSet, marker::PhantomData};
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

pub trait RoomsStorage<I: Iterator<Item = Room>> {
    fn list_rooms(&self) -> I;
    fn add_room(&mut self, room_name: &str, room: Room) -> Result<&Room, String>;
    fn get_room(&self, name: &str) -> Result<&Room, String>;
    fn delete_room(&mut self, name: &str) -> Result<(), String>;
    fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<(), String>;
    fn delete_device(&mut self, room_name: &str, device_name: &str) -> Result<(), String>;
}

pub struct Home<S, I>
where
    I: Iterator<Item = Room>,
    S: RoomsStorage<I>,
{
    name: String,
    rooms_storage: S,
    _phantom_i: PhantomData<I>,
}

pub trait ObjectReporter {
    fn get_device_state(&self, room: &str, device: &str) -> Result<&str, String>;
}

impl<S, I> Home<S, I>
where
    I: Iterator<Item = Room>,
    S: RoomsStorage<I>,
{
    pub fn new(name: String, rooms_storage: S) -> Self {
        Self {
            name,
            rooms_storage,
            _phantom_i: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn list_rooms(&self) -> I {
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
        for room in rooms_iter {
            let devices = room.get_devices();
            for device in devices {
                let state = reporter.get_device_state(&room.name, device.as_str());
                match state {
                    Ok(state) => report.push_str(
                        format!(
                            "Room {}, has device {} with state - {}",
                            &room.name, device, state,
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
