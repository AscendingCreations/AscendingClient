use crate::{socket::*, Entity, Position, fade::*};
use bytey::ByteBuffer;
use phf::phf_map;

type PacketFunction = fn(&mut Socket, &mut World, &mut DrawSetting, &mut Content, &mut Alert, &mut ByteBuffer) -> Result<()>;

static PACKET_MAP: phf::Map<u32, PacketFunction> = phf_map! {
    0u32 => handle_ping,
    1u32 => handle_status,
    2u32 => handle_alertmsg,
    3u32 => handle_fltalert,
    4u32 => handle_loginok,
    5u32 => handle_ingame,
    6u32 => handle_updatemap,
    7u32 => handle_mapitems,
    8u32 => handle_mapitemsunload,
    9u32 => handle_playerdata,
    10u32 => handle_playerspawn,
    11u32 => handle_playermove,
    12u32 => handle_playermapswap,
    13u32 => handle_dataremovelist,
    14u32 => handle_dataremove,
    15u32 => handle_playerdir, 
    16u32 => handle_playervitals,
    17u32 => handle_playerinv,
    18u32 => handle_playerinvslot,
    19u32 => handle_keyinput,
    20u32 => handle_playerattack, 
    21u32 => handle_playerequipment,
    22u32 => handle_playeraction,
    23u32 => handle_playerlevel,
    24u32 => handle_playermoney,
    25u32 => handle_playerstun,
    26u32 => handle_playervariables,
    27u32 => handle_playervariable,
    28u32 => handle_playerdeath,
    29u32 => handle_npcdeath,
    30u32 => handle_playerpvp,
    31u32 => handle_playerpk,
    32u32 => handle_playeremail,
    33u32 => handle_npcunload,
    34u32 => handle_npcdata,
    35u32 => handle_npcmove,
    36u32 => handle_npcdir,
    37u32 => handle_npcvital, 
    38u32 => handle_npcattack,
    39u32 => handle_npcstun, 
    40u32 => handle_chatmsg, 
    41u32 => handle_sound, 
    42u32 => handle_target, 
    43u32 => handle_synccheck, 
    44u32 => handle_playerunload, 
    45u32 => handle_loadstatus, 
};

pub fn handle_data(socket: &mut Socket, world: &mut World, systems: &mut DrawSetting, content: &mut Content, alert: &mut Alert, data: &mut ByteBuffer) -> Result<()> {
    let id: u32 = data.read()?;

    if id > 80 {
        return Err(AscendingError::InvalidPacket);
    }

    println!("id: {id}");

    let fun = match PACKET_MAP.get(&id) {
        Some(fun) => fun,
        None => return Err(AscendingError::InvalidPacket),
    };

    fun(socket, world, systems, content, alert, data)
}

fn handle_ping(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    Ok(())
}

fn handle_status(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    Ok(())
}

fn handle_alertmsg(
    _socket: &mut Socket, 
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

fn handle_fltalert(
    _socket: &mut Socket, 
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

fn handle_loginok(
    _socket: &mut Socket, 
    _world: &mut World,
    systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    let _hour = data.read::<u32>()?;
    let _min = data.read::<u32>()?;

    println!("Login Ok");
    systems.fade.init_fade(&mut systems.gfx, FadeType::In, FADE_LOGIN);
    
    Ok(())
}

fn handle_ingame(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_updatemap(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_mapitems(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {

    let _item_entity = data.read::<Entity>()?;
    
    Ok(())
}

fn handle_mapitemsunload(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerdata(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerspawn(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playermove(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    let _entity = data.read::<Entity>()?;
    let _pos = data.read::<Position>()?;
    let _dir = data.read::<u8>()?;
    let _warp = data.read::<bool>()?;
    
    Ok(())
}

fn handle_playermapswap(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_dataremovelist(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_dataremove(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerdir(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {

    let _dir = data.read::<u8>()?;
    
    Ok(())
}

fn handle_playervitals(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerinv(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerinvslot(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_keyinput(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerattack(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerequipment(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playeraction(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerlevel(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playermoney(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerstun(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playervariables(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playervariable(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerdeath(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_npcdeath(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerpvp(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerpk(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playeremail(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_npcunload(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_npcdata(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_npcmove(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_npcdir(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_npcvital(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_npcattack(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_npcstun(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_chatmsg(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_sound(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_target(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_synccheck(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_playerunload(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

fn handle_loadstatus(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}