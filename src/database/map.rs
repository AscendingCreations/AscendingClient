use graphics::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::path::Path;

use bytey::{ByteBuffer, ByteBufferError, ByteBufferRead, ByteBufferWrite};

use crate::{data_types::*, Result, SystemHolder};

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Default,
    ByteBufferRead,
    ByteBufferWrite,
)]
pub struct WarpData {
    pub map_x: i32,
    pub map_y: i32,
    pub map_group: u64,
    pub tile_x: u32,
    pub tile_y: u32,
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Default,
    ByteBufferRead,
    ByteBufferWrite,
)]
pub struct ItemSpawnData {
    pub index: u32,
    pub amount: u16,
    pub timer: u64,
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    ByteBufferRead,
    ByteBufferWrite,
)]
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
    Copy,
    Clone,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    Default,
    Debug,
    ByteBufferRead,
    ByteBufferWrite,
)]
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

#[derive(
    Clone, Debug, Serialize, Deserialize, ByteBufferRead, ByteBufferWrite,
)]
pub struct Tile {
    pub id: Vec<u32>,
}

#[derive(
    Clone, Debug, Serialize, Deserialize, ByteBufferRead, ByteBufferWrite,
)]
pub struct MapData {
    pub position: MapPosition,
    pub tile: Vec<Tile>,
    pub dir_block: Vec<u8>,
    pub attribute: Vec<MapAttribute>,
    pub zonespawns: [Vec<(u16, u16)>; 5],
    pub zones: [(u64, [Option<u64>; 5]); 5],
    pub music: Option<String>,
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
            dir_block: vec![0; 1024],
            attribute: vec![MapAttribute::Blocked; 1024],
            zonespawns: Default::default(),
            zones: Default::default(),
            music: None,
            weather: Weather::default(),
        }
    }
}

pub fn load_file(x: i32, y: i32, group: u64) -> Result<MapData> {
    if !is_map_exist(x, y, group) {
        println!("Map does not exist");
        return Ok(MapData::default(x, y, group));
    }

    let name = format!("./data/maps/{}_{}_{}.bin", x, y, group);
    match OpenOptions::new().read(true).open(&name) {
        Ok(mut file) => {
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;

            let mut buf = ByteBuffer::new()?;
            buf.write(data)?;
            buf.move_cursor_to_start();

            buf.move_cursor(8)?;
            Ok(buf.read::<MapData>()?)
        }
        Err(e) => {
            println!("Failed to load {}, Err {:?}", name, e);
            Ok(MapData::default(x, y, group))
        }
    }
}

pub fn is_map_exist(x: i32, y: i32, group: u64) -> bool {
    let name = format!("./data/maps/{}_{}_{}.bin", x, y, group);
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
