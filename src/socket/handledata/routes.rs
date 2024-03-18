use hecs::World;
use bytey::ByteBuffer;
use graphics::*;
use crate::{add_npc, content::game_content::player::*, dir_to_enum, entity::*, fade::*, npc_finalized, set_npc_frame, socket::error::*, unload_mapitems, unload_npc, update_camera, Alert, Content, DrawSetting, EntityType, Position, Socket, NPC_SPRITE_FRAME_X, VITALS_MAX};

pub fn handle_ping(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    Ok(())
}

pub fn handle_status(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    Ok(())
}

pub fn handle_alertmsg(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
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
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
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
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
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
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_updatemap(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_mapitems(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {

    let _item_entity = data.read::<Entity>()?;
    
    Ok(())
}

pub fn handle_myindex(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    let entity = data.read::<Entity>()?;
    content.game_content.myentity = Some(entity);
    Ok(())
}

pub fn handle_playerdata(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    if let Some(entity) = content.game_content.myentity {
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
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
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

        if let Some(myentity) = content.game_content.myentity {
            if myentity != entity {
                if !world.contains(entity.0) {
                    let client_map = world.get_or_panic::<Position>(&myentity).map;
                    let player = add_player(world, systems, pos, client_map, Some(&entity));
                    content.game_content.players.insert(player);

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

                    if content.game_content.finalized {
                        player_finalized(world, systems, &player);
                    }
                }
            }
        }
    }
    
    Ok(())
}

pub fn handle_playermove(
    _socket: &mut Socket,
    world: &mut World,
    _systems: &mut DrawSetting,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let pos = data.read::<Position>()?;
        let _warp = data.read::<bool>()?;
        let _switch = data.read::<bool>()?;
        let dir = data.read::<u8>()?;

        if let Some(myentity) = content.game_content.myentity {
            if myentity != entity && world.contains(entity.0) {
                let mut movementbuffer = world.get::<&mut MovementBuffer>(entity.0).expect("Could not find MovementBuffer");
                let movement_data = MovementData { end_pos: pos, dir };
                if movementbuffer.data.is_empty() {
                    movementbuffer.data.push_back(movement_data);
                } else {
                    if let Some(data) =  movementbuffer.data.back() {
                        if *data != movement_data {
                            movementbuffer.data.push_back(movement_data);
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

pub fn handle_playerwarp(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        println!("Player Warp~~~~~~");
        let entity = data.read::<Entity>()?;
        let pos = data.read::<Position>()?;

        if world.contains(entity.0) {
            *world.get::<&mut Position>(entity.0).expect("Could not find Position") = pos;
            world.get::<&mut Movement>(entity.0).expect("Could not find movement").is_moving = false;
            world.get::<&mut PositionOffset>(entity.0).expect("Could not find PositionOffset").offset = Vec2::new(0.0, 0.0);
            let frame = world.get_or_panic::<Dir>(&entity).0 * PLAYER_SPRITE_FRAME_X as u8;
            set_player_frame(world, systems, &entity, frame as usize);
        }
        if let Some(myentity) = content.game_content.myentity {
            if myentity == entity {
                update_camera(world, &mut content.game_content, systems);
            }
        }
    }
    
    Ok(())
}

pub fn handle_playermapswap(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_dataremovelist(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {

    let remove_list = data.read::<Vec<Entity>>()?;

    remove_list.iter().for_each(|entity| {
        let world_entity_type = world.get_or_panic::<WorldEntityType>(entity);
        match world_entity_type {
            WorldEntityType::Player => {
                unload_player(world, systems, entity);
            }
            WorldEntityType::Npc => {
                unload_npc(world, systems, entity);
            }
            WorldEntityType::MapItem => {
                unload_mapitems(world, systems, entity);
            }
            _ => {}
        }
    });
    
    
    Ok(())
}

pub fn handle_dataremove(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playerdir(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {

    let _dir = data.read::<u8>()?;
    
    Ok(())
}

pub fn handle_playervitals(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playerinv(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playerinvslot(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_keyinput(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playerattack(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    seconds: f32,
) -> SocketResult<()> {
    let entity = data.read::<Entity>()?;

    if let Some(myentity) = content.game_content.myentity {
        if myentity != entity {
            let world_entity_type = world.get_or_panic::<WorldEntityType>(&entity);
            match world_entity_type {
                WorldEntityType::Player => init_player_attack(world, systems, &entity, seconds),
                _ => {}
            }
        }
    }

    Ok(())
}

pub fn handle_playerequipment(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playeraction(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playerlevel(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playermoney(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playerstun(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playervariables(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playervariable(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playerdeath(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_npcdeath(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playerpvp(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playerpk(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_playeremail(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_npcdata(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut DrawSetting,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let dir = data.read::<u8>()?;
        let hidden = data.read::<bool>()?;
        let level = data.read::<i32>()?;
        let deathtype = data.read::<DeathType>()?;
        let mode = data.read::<NpcMode>()?;
        let num = data.read::<u64>()?;
        let pdamage = data.read::<u32>()?;
        let pdefense = data.read::<u32>()?;
        let pos = data.read::<Position>()?;
        let sprite = data.read::<u16>()?;
        let mut vitals = [0; VITALS_MAX];
        vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>().expect("Could not read data"));
        let mut vitalmax = [0; VITALS_MAX];
        vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>().expect("Could not read data"));

        if let Some(myentity) = content.game_content.myentity {
            if !world.contains(entity.0) {
                let client_map = world.get_or_panic::<Position>(&myentity).map;
                let npc = add_npc(world, systems, pos, client_map, Some(&entity));
                content.game_content.npcs.insert(npc);

                {
                    world.get::<&mut Dir>(entity.0).expect("Could not find Dir").0
                        = dir;
                    world.get::<&mut Hidden>(entity.0).expect("Could not find Hidden").0
                        = hidden;
                    world.get::<&mut Level>(entity.0).expect("Could not find Level").0
                        = level;
                    *world.get::<&mut DeathType>(entity.0).expect("Could not find DeathType")
                        = deathtype;
                    *world.get::<&mut NpcMode>(entity.0).expect("Could not find NpcMode")
                        = mode;
                    world.get::<&mut NpcIndex>(entity.0).expect("Could not find NpcIndex").0
                        = num;
                    if let Ok(mut physical) = world.get::<&mut Physical>(entity.0) {
                        physical.damage = pdamage;
                        physical.defense = pdefense;
                    }
                    *world.get::<&mut Position>(entity.0).expect("Could not find Position")
                        = pos;
                    world.get::<&mut SpriteImage>(entity.0).expect("Could not find SpriteIndex").0
                        = sprite as u8;
                    if let Ok(mut vital) = world.get::<&mut Vitals>(entity.0) {
                        vital.vital = vitals;
                        vital.vitalmax = vitalmax;
                    }
                }

                if content.game_content.finalized {
                    npc_finalized(world, systems, &npc);
                }
            }
        }
    }

    Ok(())
}

pub fn handle_npcmove(
    _socket: &mut Socket,
    world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let pos = data.read::<Position>()?;
        let _warp = data.read::<bool>()?;
        let _switch = data.read::<bool>()?;
        let dir = data.read::<u8>()?;

        if world.contains(entity.0) {
            let mut movementbuffer = world.get::<&mut MovementBuffer>(entity.0).expect("Could not find MovementBuffer");
            let movement_data = MovementData { end_pos: pos, dir };
            if movementbuffer.data.is_empty() {
                movementbuffer.data.push_back(movement_data);
            } else {
                if let Some(data) =  movementbuffer.data.back() {
                    if *data != movement_data {
                        movementbuffer.data.push_back(movement_data);
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn handle_npcwarp(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let pos = data.read::<Position>()?;

        if world.contains(entity.0) {
            *world.get::<&mut Position>(entity.0).expect("Could not find Position") = pos;
            world.get::<&mut Movement>(entity.0).expect("Could not find movement").is_moving = false;
            world.get::<&mut PositionOffset>(entity.0).expect("Could not find PositionOffset").offset = Vec2::new(0.0, 0.0);
            let frame = world.get_or_panic::<Dir>(&entity).0 * NPC_SPRITE_FRAME_X as u8;
            set_npc_frame(world, systems, &entity, frame as usize);
        }
    }
    
    Ok(())
}

pub fn handle_npcdir(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_npcvital(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_npcattack(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_npcstun(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_chatmsg(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_sound(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_target(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_synccheck(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}

pub fn handle_entityunload(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;

        if world.contains(entity.0) {
            let world_entity_type = world.get_or_panic::<WorldEntityType>(&entity);
            match world_entity_type {
                WorldEntityType::Player => {
                    unload_player(world, systems, &entity);
                }
                WorldEntityType::Npc => {
                    unload_npc(world, systems, &entity);
                }
                WorldEntityType::MapItem => {
                    unload_mapitems(world, systems, &entity);
                }
                _ => {}
            }
        }
    }
    
    Ok(())
}

pub fn handle_loadstatus(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
) -> SocketResult<()> {
    
    Ok(())
}