use std::{collections::HashMap, sync::Mutex};

use log::info;

pub struct AppState {
    pub counter: Mutex<u32>,
    pub map: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub async fn insert(self: &Self, k: String, v: String) {
        let mut map = self.map.lock().unwrap();
        map.insert(k, v);
    }

    pub async fn get(self: &Self, k: &String) -> Option<String> {
        let map = self.map.lock().unwrap();
        map.get(k).map_or_else(|| None, |v| Some(v.to_owned()))
    }

    pub async fn inc(self: &Self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        info!("{:?}", counter);
    }

    pub async fn dec(self: &Self) {
        let mut counter = self.counter.lock().unwrap();
        *counter -= 1;
        info!("{:?}", counter);
    }
}
