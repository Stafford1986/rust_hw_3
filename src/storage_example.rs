use crate::{ObjectReporter, Room, RoomsStorage};
use std::collections::HashMap;

pub trait DeviceItem {
    fn get_name(&self) -> &str;
    fn get_state(&self) -> &str;
}

pub struct DeviceProvider<T: DeviceItem> {
    room_device_map: HashMap<String, Vec<T>>,
}

impl<T: DeviceItem> ObjectReporter for DeviceProvider<T> {
    fn get_device_state(&self, room: &str, device: &str) -> Result<&str, String> {
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

pub struct InmemoryRoomsStorage {
    rooms: HashMap<String, Room>,
}

pub struct RoomsIterator<'a> {
    used: Vec<&'a str>,
    rooms: &'a HashMap<String, Room>
}

impl<'a> RoomsIterator<'a> {
    fn new(rooms_map: &'a HashMap<String, Room>) -> Self {
        RoomsIterator { used: Vec::with_capacity(rooms_map.len()), rooms: rooms_map}
    }
}

impl<'a> Iterator for RoomsIterator<'a> {
    type Item = (&'a String, &'a Room);
    fn next(&mut self) -> Option<Self::Item> {
        for (k, v) in self.rooms {
            if self.used.contains(&k.as_str()) {
                continue;
            }
            self.used.push(k.as_str());
            
            return Some((k, v))
        }

        None
    }
}

impl <'a> RoomsStorage<'a> for InmemoryRoomsStorage {
    //type IterType = Iterator<Item = (&'a String, &'a Room)>;
    type IterType = RoomsIterator<'a>;
    fn list_rooms(&self) -> Self::IterType {
      RoomsIterator::new(&self.rooms)
    }
    fn add_room(&mut self, room_name: &str, room: Room) -> Result<&Room, String> {
        let err_insert = Err(format!("cat't add room {}. already exists", room_name));
        let exists = self.rooms.get(room_name).is_some();
        if exists {
            return err_insert;
        }

        let inserted = match self.rooms.insert(room_name.to_string(), room) {
            Some(_) => return Err(format!("cat't add room {}. already exists", room_name)),
            None => self.rooms.get(room_name),
        };

        if let Some(result) = inserted {
            return Ok(result);
        }

        err_insert
    }
    fn get_room(&self, name: &str) -> Result<&Room, String> {
        match self.rooms.get(name) {
            Some(room) => Ok(room),
            None => Err(format!("room {} not found", name)),
        }
    }
    fn delete_room(&mut self, name: &str) -> Result<(), String> {
        match self.rooms.remove(name) {
            Some(_room) => Ok(()),
            None => Err(format!("room {} not found", name)),
        }
    }
    fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<(), String> {
        let room = match self.rooms.get_mut(room_name) {
            Some(room) => room,
            None => return Err(format!("room {} not found", room_name)),
        };
        let is_success = room.devices.insert(device_name.to_string());
        if is_success {
            return Ok(());
        }

        Err(format!(
            "device with name {} already exists in room {}",
            device_name, room_name
        ))
    }
    fn delete_device(&mut self, room_name: &str, device_name: &str) -> Result<(), String> {
        let room = match self.rooms.get_mut(room_name) {
            Some(room) => room,
            None => return Err(format!("room {} not found", room_name)),
        };
        let is_success = room.devices.remove(device_name);
        if is_success {
            return Ok(());
        }

        Err(format!(
            "device with name {} not found in room {}",
            device_name, room_name
        ))
    }
}
