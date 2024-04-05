use crate::values::MAX_SHOP_ITEM;
use educe::Educe;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct ShopItem {
    pub index: u16,
    pub amount: u16,
    pub price: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShopData {
    pub name: String,
    pub max_item: u16,
    pub item: [ShopItem; MAX_SHOP_ITEM],
}

pub fn get_shop() -> Vec<ShopData> {
    let mut shop_data: Vec<ShopData> = Vec::new();

    let mut count = 0;
    let mut got_data = true;

    while got_data {
        if let Some(data) = load_file(count) {
            shop_data.push(data);
            count += 1;
            got_data = true;
        } else {
            got_data = false;
        }
    }

    shop_data
}

fn load_file(id: usize) -> Option<ShopData> {
    let name = format!("./data/shops/{}.json", id);

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
