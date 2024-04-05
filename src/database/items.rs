use crate::data_types::*;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemData {
    pub name: String,
    pub levelreq: u16,
    pub soundid: u16,
    pub sprite: u16,
    pub animation: Option<u32>,
    pub data: [i16; 20],
    pub itemtype: ItemTypes,
    pub itemtype2: u8,
    pub breakable: bool,
    pub stackable: bool,
    pub stacklimit: u16,
    pub baseprice: u64,
    pub repairable: bool,
    pub rgba: Rgba,
    pub sound_index: Option<String>,
}

pub fn get_item() -> Vec<ItemData> {
    let mut item_data: Vec<ItemData> = Vec::new();

    let mut count = 0;
    let mut got_data = true;

    while got_data {
        if let Some(data) = load_file(count) {
            item_data.push(data);
            count += 1;
            got_data = true;
        } else {
            got_data = false;
        }
    }

    item_data
}

fn load_file(id: usize) -> Option<ItemData> {
    let name = format!("./data/items/{}.json", id);

    match OpenOptions::new().read(true).open(name) {
        Ok(file) => {
            let reader = BufReader::new(file);

            match serde_json::from_reader(reader) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}
