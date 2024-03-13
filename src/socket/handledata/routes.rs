use hecs::World;
use bytey::ByteBuffer;
use crate::{content::game_content::player::*, dir_to_enum, entity::*, fade::*, socket::error::*, Alert, Content, DrawSetting, Position, Socket, VITALS_MAX};

pub fn handle_ping(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    Ok(())
}

pub fn handle_status(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    Ok(())
}

pub fn handle_alertmsg(
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

pub fn handle_fltalert(
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

pub fn handle_loginok(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    let _hour = data.read::<u32>()?;
    let _min = data.read::<u32>()?;

    systems.fade.init_fade(&mut systems.gfx, FadeType::In, FADE_SWITCH_TO_GAME, FadeData::None);
    Ok(())
}

pub fn handle_ingame(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_updatemap(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_mapitems(
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

pub fn handle_mapitemsunload(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_myindex(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    let entity = data.read::<Entity>()?;
    content.game_content.myentity = Some(entity);
    println!("Got Entity {:?}", content.game_content.myentity);
    
    Ok(())
}

pub fn handle_playerdata(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    if let Some(entity) = content.game_content.myentity {
        println!("Received Data {:?}", entity);

        let username = data.read::<String>()?;
        let useraccess = data.read::<UserAccess>()?;
        let dir = data.read::<u8>()?;
        let equipment = data.read::<Equipment>()?;
        let hidden = data.read::<bool>()?;
        let level = data.read::<i32>()?;
        let deathtype = data.read::<DeathType>()?;
        let pdamage = data.read::<u32>()?;
        let pdefense = data.read::<u32>()?;
        let pos = data.read::<Position>()?;
        let pk = data.read::<bool>()?;
        let pvpon = data.read::<bool>()?;
        let sprite = data.read::<u8>()?;
        let mut vitals = [0; VITALS_MAX];
        vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>().expect("Could not read data"));
        let mut vitalmax = [0; VITALS_MAX];
        vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>().expect("Could not read data"));

        if !world.contains(entity.0) {
            let player = add_player(world, systems, pos, pos.map, Some(&entity));
            content.game_content.players.insert(player);
        }

        {
            world.get::<&mut EntityName>(entity.0).expect("Could not find EntityName").0
                = username;
            *world.get::<&mut UserAccess>(entity.0).expect("Could not find UserAccess")
                = useraccess;
            world.get::<&mut Dir>(entity.0).expect("Could not find Dir").0
                = dir;
            *world.get::<&mut Equipment>(entity.0).expect("Could not find Equipment")
                = equipment;
            world.get::<&mut Hidden>(entity.0).expect("Could not find Hidden").0
                = hidden;
            world.get::<&mut Level>(entity.0).expect("Could not find Level").0
                = level;
            *world.get::<&mut DeathType>(entity.0).expect("Could not find DeathType")
                = deathtype;
            if let Ok(mut physical) = world.get::<&mut Physical>(entity.0) {
                physical.damage = pdamage;
                physical.defense = pdefense;
            }
            *world.get::<&mut Position>(entity.0).expect("Could not find Position")
                = pos;
            if let Ok(mut pvp) = world.get::<&mut PlayerPvP>(entity.0) {
                pvp.pk = pk;
                pvp.pvpon = pvpon;
            }
            world.get::<&mut SpriteImage>(entity.0).expect("Could not find SpriteImage").0
                = sprite;
            if let Ok(mut vital) = world.get::<&mut Vitals>(entity.0) {
                vital.vital = vitals;
                vital.vitalmax = vitalmax;
            }
        }
    }
    Ok(())
}

pub fn handle_playerspawn(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    println!("Received player spawn");

    let entity = data.read::<Entity>()?;

    if let Some(myentity) = content.game_content.myentity {
        if myentity == entity && world.contains(entity.0) {
            return Ok(());
        }
    }

    let username = data.read::<String>()?;
    let useraccess = data.read::<UserAccess>()?;
    let dir = data.read::<u8>()?;
    let equipment = data.read::<Equipment>()?;
    let hidden = data.read::<bool>()?;
    let level = data.read::<i32>()?;
    let deathtype = data.read::<DeathType>()?;
    let pdamage = data.read::<u32>()?;
    let pdefense = data.read::<u32>()?;
    let pos = data.read::<Position>()?;
    let pk = data.read::<bool>()?;
    let pvpon = data.read::<bool>()?;
    let sprite = data.read::<u8>()?;
    let mut vitals = [0; VITALS_MAX];
    vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>().expect("Could not read data"));
    let mut vitalmax = [0; VITALS_MAX];
    vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>().expect("Could not read data"));

    if !world.contains(entity.0) {
        let client_map = if let Some(data) = &content.game_content.myentity {
            world.get_or_panic::<Position>(data).map
        } else {
            MapPosition::default()
        };
        let player = add_player(world, systems, pos, client_map, Some(&entity));
        content.game_content.players.insert(player);
    }

    {
        world.get::<&mut EntityName>(entity.0).expect("Could not find EntityName").0
            = username;
        *world.get::<&mut UserAccess>(entity.0).expect("Could not find UserAccess")
            = useraccess;
        world.get::<&mut Dir>(entity.0).expect("Could not find Dir").0
            = dir;
        *world.get::<&mut Equipment>(entity.0).expect("Could not find Equipment")
            = equipment;
        world.get::<&mut Hidden>(entity.0).expect("Could not find Hidden").0
            = hidden;
        world.get::<&mut Level>(entity.0).expect("Could not find Level").0
            = level;
        *world.get::<&mut DeathType>(entity.0).expect("Could not find DeathType")
            = deathtype;
        if let Ok(mut physical) = world.get::<&mut Physical>(entity.0) {
            physical.damage = pdamage;
            physical.defense = pdefense;
        }
        *world.get::<&mut Position>(entity.0).expect("Could not find Position")
            = pos;
        if let Ok(mut pvp) = world.get::<&mut PlayerPvP>(entity.0) {
            pvp.pk = pk;
            pvp.pvpon = pvpon;
        }
        world.get::<&mut SpriteImage>(entity.0).expect("Could not find SpriteIndex").0
            = sprite;
        if let Ok(mut vital) = world.get::<&mut Vitals>(entity.0) {
            vital.vital = vitals;
            vital.vitalmax = vitalmax;
        }
    }
    
    Ok(())
}

pub fn handle_playermove(
    socket: &mut Socket,
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer
) -> Result<()> {
    println!("Receiving Movement");

    let entity = data.read::<Entity>()?;

    if let Some(myentity) = content.game_content.myentity {
        if myentity == entity && world.contains(entity.0) {
            return Ok(());
        }
    }
    
    let pos = data.read::<Position>()?;
    let dir = data.read::<u8>()?;

    move_player(world, systems, socket, &entity, &mut content.game_content, &dir_to_enum(dir), Some(pos));
    
    Ok(())
}

pub fn handle_playerwarp(
    _socket: &mut Socket,
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
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_dataremovelist(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_dataremove(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerdir(
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

pub fn handle_playervitals(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerinv(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerinvslot(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_keyinput(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerattack(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerequipment(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playeraction(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerlevel(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playermoney(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerstun(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playervariables(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playervariable(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerdeath(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcdeath(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerpvp(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerpk(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playeremail(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcunload(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcdata(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcmove(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcdir(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcvital(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcattack(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_npcstun(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_chatmsg(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_sound(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_target(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_synccheck(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_playerunload(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}

pub fn handle_loadstatus(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer
) -> Result<()> {
    
    Ok(())
}