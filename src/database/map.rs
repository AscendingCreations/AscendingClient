use crate::content::MAP_SIZE;
use crate::{
    BufferTask, BufferTaskEnum, MapAttributes, MapDirBlock, MapPosition,
    Result, SystemHolder, data_types::*, socket::*,
};
use graphics::*;
use log::{error, info};
use serde::{Deserialize, Serialize};
use snafu::Backtrace;
use speedy::{Endianness, Readable, Writable};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Default,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
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
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub struct ItemSpawnData {
    pub index: u32,
    pub amount: u16,
    pub timer: u64,
}

#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum MapAttribute {
    #[default]
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
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
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

#[derive(Clone, Debug)]
pub struct MapSlotData {
    pub mappos: MapPosition,
    pub map: Map,
    pub enable: bool,
    pub dir_block: MapDirBlock,
    pub attributes: MapAttributes,
    pub music: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
    Default,
)]
pub struct Tile {
    pub id: Vec<u32>,
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
    Default,
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
    pub fn new(x: i32, y: i32, group: u64) -> Self {
        Self {
            position: MapPosition {
                x,
                y,
                group: group as i32,
            },
            tile: vec![Tile { id: vec![0; 1024] }; 9],
            dir_block: vec![0; 1024],
            attribute: vec![MapAttribute::Blocked; 1024],
            ..Default::default()
        }
    }
}

pub fn load_map_data(
    systems: &mut SystemHolder,
    key: Index,
    mappos: MapPosition,
) -> Result<()> {
    if let Some(mapslotdata) = systems.base.mapdata.get_mut(key) {
        let mut buffer = Vec::with_capacity(131_072);
        let mapdata =
            load_file(mappos.x, mappos.y, mappos.group as u64, &mut buffer)?;

        (0..32).for_each(|x| {
            (0..32).for_each(|y| {
                let tile_num = get_tile_pos(x, y);
                (0..9).for_each(|layer| {
                    let id = mapdata.tile[layer].id[tile_num] as usize;
                    if id > 0 {
                        mapslotdata.map.set_tile(
                            UVec3::new(x as u32, y as u32, layer as u32),
                            TileData {
                                id,
                                color: Color::rgba(255, 255, 255, 255),
                                anim_time: 250,
                            },
                        );
                    }
                });
            });
        });

        mapslotdata.dir_block = MapDirBlock {
            dir: mapdata.dir_block,
        };
        mapslotdata.attributes = MapAttributes {
            attribute: mapdata.attribute,
        };
        mapslotdata.music = mapdata.music;
    }
    Ok(())
}

pub fn create_map_data(
    systems: &mut SystemHolder,
    map_renderer: &mut MapRenderer,
    mappos: MapPosition,
    world_pos: Vec2,
) -> Result<Index> {
    if let Some(mut map) = Map::new(
        &mut systems.renderer,
        map_renderer,
        TILE_SIZE as u32,
        world_pos,
        MapZLayers::default(),
    ) {
        map.can_render = true;

        let mapslotdata = MapSlotData {
            mappos,
            map,
            enable: false,
            dir_block: MapDirBlock::default(),
            attributes: MapAttributes::default(),
            music: None,
        };

        Ok(systems.base.mapdata.insert(mapslotdata))
    } else {
        Err(ClientError::MapCreationFailed {
            backtrace: Backtrace::new(),
        })
    }
}

pub fn get_map_key(
    systems: &mut SystemHolder,
    map_renderer: &mut MapRenderer,
    mappos: MapPosition,
    buffer: &mut BufferTask,
    center_pos: MapPosition,
) -> Result<Index> {
    let center_map_pos = if let Some(index) =
        systems.base.mappos_key.get(&center_pos)
        && let Some(mapdata) = systems.base.mapdata.get(*index)
    {
        mapdata.map.pos
    } else {
        Vec2::ZERO
    };

    let world_pos = center_map_pos
        + Vec2::new(
            (mappos.x - center_pos.x) as f32 * MAP_SIZE.x,
            (mappos.y - center_pos.y) as f32 * MAP_SIZE.y,
        );

    if let Some(index) = systems.base.mappos_key.get(&mappos) {
        systems.base.map_cache.promote(index);
        if let Some(mapdata) = systems.base.mapdata.get_mut(*index)
            && mapdata.map.pos != world_pos
        {
            mapdata.map.set_pos(world_pos);
        }
        return Ok(*index);
    }

    if systems.base.map_cache.len() > 60 {
        let keydata = systems.base.map_cache.pop_lru();
        if let Some(key) = keydata {
            if let Some(mapdata) = systems.base.mapdata.get(key.0) {
                let mappos = mapdata.mappos;
                systems.base.mappos_key.remove(&mappos);
            }

            systems.base.mapdata.remove(key.0);
        }
    }

    let key = create_map_data(systems, map_renderer, mappos, world_pos)?;
    systems.base.mappos_key.insert(mappos, key);
    systems.base.map_cache.push(key, key);
    buffer.add_task(BufferTaskEnum::ApplyMap(mappos, key));
    Ok(key)
}

pub fn clear_map_data(
    systems: &mut SystemHolder,
    map_renderer: &mut MapRenderer,
) {
    for mapslotdata in systems.base.mapdata.iter_mut() {
        mapslotdata
            .1
            .map
            .unload(&mut systems.renderer, map_renderer);
    }

    systems.base.mapdata.clear();
    systems.base.mappos_key.clear();
}

pub fn set_map_visible(systems: &mut SystemHolder, key: Index, visible: bool) {
    if let Some(mapslotdata) = systems.base.mapdata.get_mut(key) {
        mapslotdata.map.can_render = visible;
    }
}

pub fn get_map_pos(systems: &mut SystemHolder, map_pos: MapPosition) -> Vec2 {
    if let Some(key) = systems.base.mappos_key.get(&map_pos)
        && let Some(mapslotdata) = systems.base.mapdata.get(*key)
    {
        return mapslotdata.map.pos;
    }

    Vec2::ZERO
}

pub fn get_map_dir_block(
    systems: &mut SystemHolder,
    key: Index,
) -> MapDirBlock {
    if let Some(mapslotdata) = systems.base.mapdata.get(key) {
        return mapslotdata.dir_block.clone();
    }

    MapDirBlock::default()
}

pub fn get_map_attributes(
    systems: &mut SystemHolder,
    key: Index,
) -> MapAttributes {
    if let Some(mapslotdata) = systems.base.mapdata.get(key) {
        return mapslotdata.attributes.clone();
    }

    MapAttributes::default()
}

pub fn get_map_music(systems: &mut SystemHolder, key: Index) -> Option<String> {
    if let Some(mapslotdata) = systems.base.mapdata.get(key) {
        return mapslotdata.music.clone();
    }

    None
}

pub fn get_map_id(
    systems: &mut SystemHolder,
    key: Index,
) -> Option<MapPosition> {
    if let Some(mapslotdata) = systems.base.mapdata.get(key) {
        return Some(mapslotdata.mappos);
    }

    None
}

pub fn load_file(
    x: i32,
    y: i32,
    group: u64,
    buffer: &mut Vec<u8>,
) -> Result<MapData> {
    if !is_map_exist(x, y, group) {
        return Ok(MapData::new(x, y, group));
    }

    buffer.clear();

    let name = format!("./data/maps/{x}_{y}_{group}.bin");

    match OpenOptions::new().read(true).open(&name) {
        Ok(mut file) => {
            file.read_to_end(buffer)?;
            Ok(MapData::read_from_buffer(buffer).unwrap())
        }
        Err(e) => {
            error!("Failed to load {name}, Err {e:?}");
            Ok(MapData::new(x, y, group))
        }
    }
}

pub fn is_map_exist(x: i32, y: i32, group: u64) -> bool {
    let name = format!("./data/maps/{x}_{y}_{group}.bin");
    Path::new(&name).exists()
}

pub fn get_tile_pos(x: i32, y: i32) -> usize {
    (x + (y * 32_i32)) as usize
}
