use home_work_3::devices::{DeviceItem, DeviceProvider};
use home_work_3::home::Home;
use home_work_3::home::RoomsStorage;
use home_work_3::room::Room;
use std::collections::{HashMap, HashSet};

enum Device {
    Breaker(String, String),
    Thermometr(String, i32),
}

impl DeviceItem for Device {
    fn get_name(&self) -> &str {
        match self {
            Device::Breaker(name, _state) => return name.as_str(),
            Device::Thermometr(name, _state) => return name.as_str(),
        }
    }
    fn get_state(&self) -> String {
        match self {
            Device::Breaker(_name, state) => return state.clone(),
            Device::Thermometr(_name, state) => return state.clone().to_string(),
        }
    }
}

pub struct InmemoryRoomsStorage {
    rooms: HashMap<String, Room>,
}

impl InmemoryRoomsStorage {
    fn new(rooms: HashMap<String, Room>) -> Self {
        InmemoryRoomsStorage { rooms }
    }
}

pub struct RoomsIterator<'a> {
    used: Vec<&'a str>,
    rooms: &'a HashMap<String, Room>,
}

impl<'a> RoomsIterator<'a> {
    fn new(rooms_map: &'a HashMap<String, Room>) -> Self {
        RoomsIterator {
            used: Vec::with_capacity(rooms_map.len()),
            rooms: rooms_map,
        }
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

            return Some((k, v));
        }

        None
    }
}

impl<'a> RoomsStorage<'a> for InmemoryRoomsStorage {
    type IterType = RoomsIterator<'a>;
    fn list_rooms(&'a self) -> Self::IterType {
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
        let is_success = room.insert_device(device_name.to_string());
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
        let is_success = room.remove_device(device_name);
        if is_success {
            return Ok(());
        }

        Err(format!(
            "device with name {} not found in room {}",
            device_name, room_name
        ))
    }
}

fn main() {
    let mut device_map: HashMap<String, Vec<Device>> = HashMap::new();
    let breaker = Device::Breaker("Breaker".into(), "OFF".into());
    let thermometr = Device::Thermometr("Thermometr".into(), 20);
    if let Some(_val) = device_map.insert("badroom".to_owned(), vec![breaker, thermometr]) {
        panic!("value already presents in device map")
    }

    let device_provider = DeviceProvider::new(device_map);

    let mut rooms_map = HashMap::new();
    if let Some(_val) = rooms_map.insert(
        "Bedroom".to_owned(),
        Room::new(
            "badroom".into(),
            HashSet::from(["Breaker".to_owned(), "Thermometr".to_owned()]),
        ),
    ) {
        panic!("value already presents in device map")
    }
    if let Some(_val) = rooms_map.insert(
        "Kitchen".to_owned(),
        Room::new(
            "kirchen".into(),
            HashSet::from(["Breaker".to_owned(), "fridge".to_owned()]),
        ),
    ) {
        panic!("value already presents in device map")
    }

    let inmemory_rooms_storage = InmemoryRoomsStorage::new(rooms_map);

    let home = Home::new("SmartHome".to_owned(), inmemory_rooms_storage);
    let report = home.get_report(device_provider);
    println!("{}", report)
}
