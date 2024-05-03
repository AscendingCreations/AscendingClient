use crate::{data_types::*, socket::*, Result};
use log::warn;
use serde::{Deserialize, Serialize};
use speedy::{Endianness, Readable, Writable};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};

#[derive(Clone, Debug, Deserialize, Serialize, Readable, Writable)]
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

pub fn get_item() -> Result<Vec<ItemData>> {
    let mut item_data: Vec<ItemData> = Vec::with_capacity(MAX_ITEMS);

    for i in 0..MAX_ITEMS {
        if let Some(data) = load_file(i)? {
            item_data.push(data);
        }
    }

    Ok(item_data)
}

fn load_file(id: usize) -> Result<Option<ItemData>> {
    let name = format!("./data/items/{}.bin", id);

    match OpenOptions::new().read(true).open(name) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            Ok(Some(ItemData::read_from_buffer(&bytes).unwrap()))
        }
        Err(e) => {
            warn!("Item Load File Num {} Err: {}", id, e);
            Ok(None)
        }
    }
}
