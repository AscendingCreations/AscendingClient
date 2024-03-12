use hecs::World;
use bytey::ByteBuffer;
use crate::{entity::*, fade::*, socket::error::*, content::game_content::player::*, Alert, Content, DrawSetting, Position, VITALS_MAX};

pub fn handle_ping(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    Ok(())
}

pub fn handle_status(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    Ok(())
}

pub fn handle_alertmsg(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    let message = data.read::<String>()?;
    let close = data.read::<u8>()?;

    println!("{}, should close: {}", message, close);
    Ok(())
}

pub fn handle_fltalert(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    let _flttype = data.read::<u8>()?;
    let _message = data.read::<String>()?;
    
    Ok(())
}

pub fn handle_loginok(
    _world: &mut World,
    systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    let entity = data.read::<Entity>()?;
    let _hour = data.read::<u32>()?;
    let _min = data.read::<u32>()?;
    
    systems.fade.init_fade(&mut systems.gfx, FadeType::In, FADE_SWITCH_TO_GAME, FadeData::Entity(entity));
    Ok(())
}

pub fn handle_ingame(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_updatemap(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_mapitems(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {

    let _item_entity = data.read::<Entity>()?;
    
    Ok(())
}

pub fn handle_mapitemsunload(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerspawn(
    world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    println!("Received player spawn");

    let entity = data.read::<Entity>()?;

    if !world.contains(entity.0) {

    }

    {
        world.get::<&mut EntityName>(entity.0).expect("Could not find EntityName").0
            = data.read::<String>().expect("Could not read data");
        *world.get::<&mut UserAccess>(entity.0).expect("Could not find UserAccess")
            = data.read::<UserAccess>().expect("Could not read data");
        world.get::<&mut Dir>(entity.0).expect("Could not find Dir").0
            = data.read::<u8>().expect("Could not read data");
        *world.get::<&mut Equipment>(entity.0).expect("Could not find Equipment")
            = data.read::<Equipment>().expect("Could not read data");
        world.get::<&mut Hidden>(entity.0).expect("Could not find Hidden").0
            = data.read::<bool>().expect("Could not read data");
        world.get::<&mut Level>(entity.0).expect("Could not find Level").0
            = data.read::<i32>().expect("Could not read data");
        *world.get::<&mut DeathType>(entity.0).expect("Could not find DeathType")
            = data.read::<DeathType>().expect("Could not read data");
        if let Ok(mut physical) = world.get::<&mut Physical>(entity.0) {
            physical.damage = data.read::<u32>().expect("Could not read data");
            physical.defense = data.read::<u32>().expect("Could not read data");
        }
        *world.get::<&mut Position>(entity.0).expect("Could not find Position")
            = data.read::<Position>().expect("Could not read data");
        if let Ok(mut pvp) = world.get::<&mut PlayerPvP>(entity.0) {
            pvp.pk = data.read::<bool>().expect("Could not read data");
            pvp.pvpon = data.read::<bool>().expect("Could not read data");
        }
        world.get::<&mut SpriteIndex>(entity.0).expect("Could not find SpriteIndex").0
            = data.read::<u8>().expect("Could not read data") as usize;
        if let Ok(mut vital) = world.get::<&mut Vitals>(entity.0) {
            vital.vital.copy_from_slice(&data.read::<[i32; VITALS_MAX]>().expect("Could not read data"));
            vital.vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>().expect("Could not read data"));
        }
    }
    
    Ok(())
}

pub fn handle_playermove(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    let _entity = data.read::<Entity>()?;
    let _pos = data.read::<Position>()?;
    let _dir = data.read::<u8>()?;
    
    Ok(())
}

pub fn handle_playerwarp(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    let _entity = data.read::<Entity>()?;
    let _pos = data.read::<Position>()?;
    
    Ok(())
}

pub fn handle_playermapswap(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_dataremovelist(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_dataremove(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerdir(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {

    let _dir = data.read::<u8>()?;
    
    Ok(())
}

pub fn handle_playervitals(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerinv(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerinvslot(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_keyinput(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerattack(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerequipment(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playeraction(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerlevel(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playermoney(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerstun(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playervariables(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playervariable(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerdeath(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcdeath(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerpvp(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerpk(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playeremail(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcunload(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcdata(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcmove(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcdir(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcvital(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcattack(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcstun(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_chatmsg(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_sound(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_target(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_synccheck(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerunload(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_loadstatus(
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}