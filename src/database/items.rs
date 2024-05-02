use crate::{data_types::*, socket::*, Result};
use log::warn;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};

#[derive(
    Clone, Debug, Deserialize, Serialize, ByteBufferRead, ByteBufferWrite,
)]
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
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;

            let mut buf = ByteBuffer::with_capacity(data.len())?;
            buf.write(data)?;
            buf.move_cursor_to_start();

            buf.move_cursor(8)?;
            Ok(Some(buf.read::<ItemData>()?))
        }
        Err(e) => {
            warn!("Item Load File Num {} Err: {}", id, e);
            Ok(None)
        }
    }
}
