use crate::{data_types::*, socket::*, Result};
use log::warn;
use serde::{Deserialize, Serialize};
use speedy::{Endianness, Readable, Writable};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};

#[derive(Clone, Debug, Deserialize, Serialize, Readable, Writable)]
pub struct DropItem {
    pub item: u32,
    pub amount: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Readable, Writable)]
pub struct NpcDrop {
    pub items: [DropItem; 5],
    pub shares: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Readable, Writable)]
pub struct NpcData {
    pub name: String,
    pub level: i32,
    pub sprite: i32,
    pub respawn_wait: i64,
    pub movement_wait: i64,
    pub attack_wait: i64,
    pub intervaled_wait: i64,
    pub spawn_wait: i64,
    pub maxhp: u32,
    pub maxsp: u32,
    pub maxmp: u32,
    pub sight: i32,
    pub follow_sight: i32,
    pub walkdistance: u32,
    pub pdamage: u32,
    pub pdefense: u32,
    pub canpassthru: bool,
    pub size: TileBox,
    pub behaviour: AIBehavior,
    pub maxdamage: u32,
    pub mindamage: u32,
    pub target_auto_switch: bool,
    pub target_attacked_switch: bool,
    pub target_auto_switch_chance: i64,
    pub target_range_dropout: bool,
    pub can_target: bool,
    pub can_move: bool,
    pub can_attack_player: bool,
    pub has_allys: bool,
    pub has_enemies: bool, // New
    pub can_attack: bool,
    pub has_selfonly: bool,
    pub has_friendonly: bool,
    pub has_groundonly: bool,
    pub runsaway: bool,
    pub isanimated: bool,
    pub run_damage: u32,
    pub spawntime: (GameTime, GameTime), //skill type to cast it with and  percentage needed to cast and Max Percentage.
    pub range: i32, // New       //attack range. How far they need to be to hit their target.
    pub enemies: Vec<u64>, // New //list of enemies the npcs can attack of other npc's... WAR!
    pub drops: [NpcDrop; 10],
    pub free_shares: u32,
    pub exp: i64,
}

pub fn get_npc() -> Result<Vec<NpcData>> {
    let mut npc_data: Vec<NpcData> = Vec::with_capacity(MAX_NPCS);

    for i in 0..MAX_NPCS {
        if let Some(data) = load_file(i)? {
            npc_data.push(data);
        }
    }

    Ok(npc_data)
}

fn load_file(id: usize) -> Result<Option<NpcData>> {
    let name = format!("./data/npcs/{}.bin", id);

    match OpenOptions::new().read(true).open(name) {
        Ok(mut file) => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            Ok(Some(NpcData::read_from_buffer(&bytes).unwrap()))
        }
        Err(e) => {
            warn!("Npc Load File Num {} Err: {}", id, e);
            Ok(None)
        }
    }
}
