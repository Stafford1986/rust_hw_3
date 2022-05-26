use std::collections::HashMap;

pub struct Device {
    name: String,
}

pub struct Room {
    name: String,
    devices: HashMap<String, Device>,
}

pub struct Home {
    name: String,
    rooms: HashMap<String, Room>,
}

pub trait ObjectState {
    fn object_name(&self) -> String;
    fn list_rooms(&self) -> Option<Vec<&str>>;
    fn list_devices(&self, room_name: &str) -> Option<Vec<&str>>;
    fn get_device_state(&self, room_name: &str, device_name: &str) -> String;
}

impl Home {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn object_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn list_rooms(&self) -> Option<Vec<&str>> {
        let mut rooms: Vec<&str> = Vec::new();

        for k in self.rooms.keys() {
            rooms.push(k.as_str())
        }

        if !rooms.is_empty() {
            Some(rooms)
        } else {
            None
        }
    }

    pub fn add_room(&mut self, name: String, room: Room) -> Result<(), String> {
        let res = self.rooms.get(&name);

        if res.is_some() {
            return Err("room already exists".to_owned());
        }

        let room = self.rooms.insert(name, room);
        match room {
            Some(_) => Err("room already exists".to_owned()),
            None => Ok(()),
        }
    }

    pub fn delete_room(&mut self, name: String) -> Option<Room> {
        self.rooms.remove(&name)
    }

    pub fn list_devices(&self, room_name: &str) -> Option<Vec<&str>> {
        let mut devices: Vec<&str> = Vec::new();

        if let Some(room) = self.rooms.get(room_name) {
            for k in room.devices.keys() {
                devices.push(k.as_str())
            }
            return Some(devices);
        }

        None
    }

    pub fn add_device(
        &mut self,
        room_name: String,
        device_name: String,
        device_value: Device,
    ) -> Result<(), String> {
        let room = self.rooms.get_mut(&room_name);
        match room {
            Some(room) => {
                let device = room.devices.get(&device_name);
                match device {
                    Some(_) => {
                        return Err(format!("device with name: {} already exists", &device_name))
                    }
                    None => {
                        if room
                            .devices
                            .insert(device_name.clone(), device_value)
                            .is_none()
                        {
                            return Ok(());
                        }
                        return Err(format!("device with name: {} already exists", &device_name));
                    }
                }
            }
            None => Err(format!("room with name: {} does't exist", room_name)),
        }
    }

    pub fn delete_divece(&mut self, room_name: String, device_name: String) -> Result<(), String> {
        let room = self.rooms.get_mut(&room_name);
        match room {
            Some(room) => {
                let _ = room.devices.remove(&device_name);
                Ok(())
            }
            None => return Err(format!("room with name: {} does't exist", room_name)),
        }
    }

    pub fn get_device_state(&self, room_name: &str, device_name: &str) -> String {
        let room = self.rooms.get(room_name);
        match room {
            Some(value) => {
                if let Some(device) = value.devices.get(device_name) {
                    return format!(
                        "Room with name{}, 
                        original room name - {}, 
                        has device {} with original device name - {}",
                        room_name, value.name, device_name, device.name,
                    );
                }

                "".to_owned()
            }
            None => "".to_owned(),
        }
    }
}

pub fn full_object_report<T: ObjectState>(object: T) -> String {
    let mut report = String::new();
    let list_rooms = object.list_rooms();

    let rooms = match list_rooms {
        Some(value) => value,
        None => return format!("Object {} has not rooms", object.object_name()),
    };

    report.push_str(format!("Report for: {}", object.object_name()).as_str());

    let rooms_iter = rooms.into_iter();
    for value in rooms_iter {
        let devices_list = object.list_devices(value);

        let devices = match devices_list {
            Some(value) => value,
            None => continue,
        };

        for v in devices {
            let res = object.get_device_state(value, v);
            if res.is_empty() {
                continue;
            }
            report.push_str(&res)
        }
    }

    report
}
