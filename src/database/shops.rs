use crate::{data_types::*, socket::*, Result};
use educe::Educe;
use log::warn;
use serde::{Deserialize, Serialize};
use speedy::{Endianness, Readable, Writable};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Serialize,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub struct ShopItem {
    pub index: u16,
    pub amount: u16,
    pub price: u64,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Serialize,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub struct ShopData {
    pub name: String,
    pub max_item: u16,
    pub item: [ShopItem; MAX_SHOP_ITEM],
}

pub fn get_shop() -> Result<Vec<ShopData>> {
    let mut shop_data: Vec<ShopData> = Vec::with_capacity(MAX_SHOPS);

    for i in 0..MAX_SHOPS {
        if let Some(data) = load_file(i)? {
            shop_data.push(data);
        }
    }

    Ok(shop_data)
}

fn load_file(id: usize) -> Result<Option<ShopData>> {
    let name = format!("./data/shops/{}.bin", id);

    match OpenOptions::new().read(true).open(name) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            Ok(Some(ShopData::read_from_buffer(&bytes).unwrap()))
        }
        Err(e) => {
            warn!("Shop Load File Num {} Err: {}", id, e);
            Ok(None)
        }
    }
}
