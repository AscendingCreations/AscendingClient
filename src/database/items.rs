use crate::{data_types::*, Result};
use bytey::{ByteBuffer, ByteBufferError, ByteBufferRead, ByteBufferWrite};
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
    let mut item_data: Vec<ItemData> = Vec::new();

    let mut count = 0;
    let mut got_data = true;

    while got_data {
        if let Some(data) = load_file(count)? {
            item_data.push(data);
            count += 1;
            got_data = true;
        } else {
            got_data = false;
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

            let mut buf = ByteBuffer::new()?;
            buf.write(data)?;
            buf.move_cursor_to_start();

            buf.move_cursor(8)?;
            Ok(Some(buf.read::<ItemData>()?))
        }
        Err(_) => Ok(None),
    }
}
