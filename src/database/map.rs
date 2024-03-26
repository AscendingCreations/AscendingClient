use graphics::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;

use crate::{MapPosition, SystemHolder};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct WarpData {
    pub map_x: i32,
    pub map_y: i32,
    pub map_group: u64,
    pub tile_x: u32,
    pub tile_y: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ItemSpawnData {
    pub index: u32,
    pub amount: u16,
    pub timer: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MapAttribute {
    Walkable,
    Blocked,
    NpcBlocked,
    Warp(WarpData),
    Sign(String),
    ItemSpawn(ItemSpawnData),
    Storage,
    Shop(u16),
    Count,
}

#[derive(
    Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Default, Debug,
)]
#[repr(u8)]
pub enum Weather {
    #[default]
    None,
    Rain,
    Snow,
    Sunny,
    Storm,
    Blizzard,
    Heat,
    Hail,
    SandStorm,
    Windy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub id: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapData {
    pub position: MapPosition,
    pub tile: Vec<Tile>,
    pub attribute: Vec<MapAttribute>,
    pub zonespawns: [Vec<(u16, u16)>; 5],
    pub zones: [(u64, [Option<u64>; 5]); 5],
    pub music: u32,
    pub weather: Weather,
}

impl MapData {
    pub fn default(x: i32, y: i32, group: u64) -> Self {
        Self {
            position: MapPosition {
                x,
                y,
                group: group as i32,
            },
            tile: vec![Tile { id: vec![0; 1024] }; 9],
            attribute: vec![MapAttribute::Walkable; 1024],
            zonespawns: Default::default(),
            zones: Default::default(),
            music: 0,
            weather: Weather::default(),
        }
    }
}

pub fn load_file(x: i32, y: i32, group: u64) -> MapData {
    if !is_map_exist(x, y, group) {
        println!("Map does not exist");
        return MapData::default(x, y, group);
    }

    let name = format!("./data/maps/{}_{}_{}.json", x, y, group);
    match OpenOptions::new().read(true).open(&name) {
        Ok(file) => {
            let reader = BufReader::new(file);

            match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(e) => {
                    println!("Failed to load {}, Err {:?}", name, e);
                    MapData::default(x, y, group)
                }
            }
        }
        Err(e) => {
            println!("Failed to load {}, Err {:?}", name, e);
            MapData::default(x, y, group)
        }
    }
}

pub fn is_map_exist(x: i32, y: i32, group: u64) -> bool {
    let name = format!("./data/maps/{}_{}_{}.json", x, y, group);
    Path::new(&name).exists()
}

pub fn clear_map(systems: &mut SystemHolder, map_index: usize) {
    (0..9).for_each(|layer| {
        (0..32).for_each(|x| {
            (0..32).for_each(|y| {
                systems.gfx.set_map_tile(
                    map_index,
                    (x, y, layer),
                    TileData::default(),
                );
            });
        });
    });
}

pub fn load_map_data(
    systems: &mut SystemHolder,
    mapdata: &MapData,
    map_index: usize,
) {
    clear_map(systems, map_index);
    (0..32).for_each(|x| {
        (0..32).for_each(|y| {
            let tile_num = get_tile_pos(x, y);
            (0..9).for_each(|layer| {
                let id = mapdata.tile[layer].id[tile_num] as usize;
                if id > 0 {
                    systems.gfx.set_map_tile(
                        map_index,
                        (x as u32, y as u32, layer as u32),
                        TileData {
                            id,
                            color: Color::rgba(255, 255, 255, 255),
                        },
                    );
                }
            });
        });
    });
}

pub fn get_tile_pos(x: i32, y: i32) -> usize {
    (x + (y * 32_i32)) as usize
}
