use graphics::{AscendingError, OtherError};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;
use graphics::*;

use crate::DrawSetting;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MapAttribute {
    Walkable,
    Blocked,
    Warp(i32, i32, u64, u32, u32),
    Sign(String),
    Count,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub id: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapData {
    pub x: i32,
    pub y: i32,
    pub group: u64,
    pub tile: Vec<Tile>,
    pub attribute: Vec<MapAttribute>,
    pub zonespawns: [Vec<(u16, u16)>; 5],
    pub zones: [(u64, [Option<u64>; 5]); 5],
    pub fixed_weather: u8,
}

impl MapData {
    pub fn default(x: i32, y: i32, group: u64) -> Self {
        Self {
            x,
            y,
            group,
            tile: vec![Tile { id: vec![0; 1024] }; 9],
            attribute: vec![MapAttribute::Walkable; 1024],
            zonespawns: Default::default(),
            zones: Default::default(),
            fixed_weather: 0,
        }
    }
}

pub fn load_file(
    x: i32,
    y: i32,
    group: u64,
) -> MapData {
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

pub fn clear_map(systems: &mut DrawSetting, map_index: usize) {
    (0..8).for_each(|layer| {
        (0..32).for_each(|x| {
            (0..32).for_each(|y| {
                systems.gfx
                    .set_map_tile(map_index, (x, y, layer), TileData::default());
            });
        });
    });
}

pub fn load_map_data(
    systems: &mut DrawSetting,
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