use crate::{data_types::*, Result};
use bytey::{ByteBuffer, ByteBufferError, ByteBufferRead, ByteBufferWrite};
use educe::Educe;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};

#[derive(
    Clone, Copy, Debug, Deserialize, Serialize, ByteBufferRead, ByteBufferWrite,
)]
pub struct ShopItem {
    pub index: u16,
    pub amount: u16,
    pub price: u64,
}

#[derive(
    Clone, Debug, Deserialize, Serialize, ByteBufferRead, ByteBufferWrite,
)]
pub struct ShopData {
    pub name: String,
    pub max_item: u16,
    pub item: [ShopItem; MAX_SHOP_ITEM],
}

pub fn get_shop() -> Result<Vec<ShopData>> {
    let mut shop_data: Vec<ShopData> = Vec::new();

    let mut count = 0;
    let mut got_data = true;

    while got_data {
        if let Some(data) = load_file(count)? {
            shop_data.push(data);
            count += 1;
            got_data = true;
        } else {
            got_data = false;
        }
    }

    Ok(shop_data)
}

fn load_file(id: usize) -> Result<Option<ShopData>> {
    let name = format!("./data/shops/{}.bin", id);

    match OpenOptions::new().read(true).open(name) {
        Ok(mut file) => {
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;

            let mut buf = ByteBuffer::new()?;
            buf.write(data)?;
            buf.move_cursor_to_start();

            buf.move_cursor(8)?;
            Ok(Some(buf.read::<ShopData>()?))
        }
        Err(_) => Ok(None),
    }
}
