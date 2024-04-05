use crate::{
    add_npc, close_interface,
    content::game_content::{interface::chatbox::*, player::*},
    dir_to_enum,
    entity::{self, *},
    fade::*,
    get_percent, get_start_map_pos, is_map_connected, npc_finalized,
    open_interface, player_get_armor_defense, player_get_weapon_damage,
    send_handshake, set_npc_frame, unload_mapitems, unload_npc, update_camera,
    Alert, AlertIndex, AlertType, BufferTask, ChatTask, Content,
    EncryptionState, EntityType, FtlType, IsUsingType, MapItem, MessageChannel,
    Position, ProfileLabel, Result, Socket, SystemHolder, TradeStatus, Window,
    MAX_EQPT, NPC_SPRITE_FRAME_X, VITALS_MAX,
};
use bytey::ByteBuffer;
use graphics::*;
use hecs::World;

pub fn handle_ping(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_status(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_alertmsg(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let message = data.read::<String>()?;
    let close = data.read::<u8>()?;

    println!("{}, should close: {}", message, close);
    Ok(())
}

pub fn handle_fltalert(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _flttype = data.read::<FtlType>()?;
    let _message = data.read::<String>()?;

    Ok(())
}

pub fn handle_handshake(
    socket: &mut Socket,
    world: &mut World,
    _systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    if let Some(entity) = content.game_content.myentity {
        let code = data.read::<String>()?;
        let handshake = data.read::<String>()?;
        world.spawn_at(entity.0, (ReloginCode { code },));
        socket.encrypt_state = EncryptionState::None;
        send_handshake(socket, handshake)?;
    }
    Ok(())
}

pub fn handle_loginok(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _hour = data.read::<u32>()?;
    let _min = data.read::<u32>()?;

    systems.fade.init_fade(
        &mut systems.gfx,
        FadeType::In,
        FADE_SWITCH_TO_GAME,
        FadeData::None,
    );
    Ok(())
}

pub fn handle_ingame(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_updatemap(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_mapitems(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let pos = data.read::<Position>()?;
        let item = data.read::<Item>()?;
        let _owner = data.read::<Option<Entity>>()?;
        let did_spawn = data.read::<bool>()?;

        if let Some(myentity) = content.game_content.myentity {
            if !world.contains(entity.0) {
                let client_map = world.get_or_err::<Position>(&myentity)?.map;
                let sprite = if let Some(itemdata) =
                    systems.base.item.get(item.num as usize)
                {
                    itemdata.sprite as usize
                } else {
                    0
                };
                let mapitem = MapItem::create(
                    world,
                    systems,
                    sprite,
                    pos,
                    client_map,
                    Some(&entity),
                )?;

                content.game_content.mapitems.insert(mapitem);

                if did_spawn && content.game_content.finalized {
                    MapItem::finalized(world, systems, &entity)?;
                }
            }
        }
    }

    Ok(())
}

pub fn handle_myindex(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let entity = data.read::<Entity>()?;
    content.game_content.myentity = Some(entity);
    Ok(())
}

pub fn handle_playerdata(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
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
        vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let mut vitalmax = [0; VITALS_MAX];
        vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);

        if !world.contains(entity.0) || !content.game_content.in_game {
            let player = add_player(
                world,
                systems,
                pos,
                pos.map,
                Some(&entity),
                Some(&entity),
            )?;
            content.game_content.players.insert(player);
            content.game_content.in_game = true;
        }

        content.game_content.player_data.equipment = equipment.items.clone();

        {
            world.get::<&mut EntityName>(entity.0)?.0 = username;
            *world.get::<&mut UserAccess>(entity.0)? = useraccess;
            world.get::<&mut Dir>(entity.0)?.0 = dir;
            *world.get::<&mut Equipment>(entity.0)? = equipment;
            world.get::<&mut Hidden>(entity.0)?.0 = hidden;
            world.get::<&mut Level>(entity.0)?.0 = level;
            *world.get::<&mut DeathType>(entity.0)? = deathtype;
            if let Ok(mut physical) = world.get::<&mut Physical>(entity.0) {
                physical.damage = pdamage;
                physical.defense = pdefense;
            }
            *world.get::<&mut Position>(entity.0)? = pos;
            if let Ok(mut pvp) = world.get::<&mut PlayerPvP>(entity.0) {
                pvp.pk = pk;
                pvp.pvpon = pvpon;
            }
            world.get::<&mut SpriteImage>(entity.0)?.0 = sprite;
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
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
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
        vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let mut vitalmax = [0; VITALS_MAX];
        vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let did_spawn = data.read::<bool>()?;

        if let Some(myentity) = content.game_content.myentity {
            if myentity != entity && !world.contains(entity.0) {
                let client_map = world.get_or_err::<Position>(&myentity)?.map;
                let player = add_player(
                    world,
                    systems,
                    pos,
                    client_map,
                    Some(&entity),
                    None,
                )?;
                content.game_content.players.insert(player);

                {
                    world.get::<&mut EntityName>(entity.0)?.0 = username;
                    *world.get::<&mut UserAccess>(entity.0)? = useraccess;
                    world.get::<&mut Dir>(entity.0)?.0 = dir;
                    *world.get::<&mut Equipment>(entity.0)? = equipment;
                    world.get::<&mut Hidden>(entity.0)?.0 = hidden;
                    world.get::<&mut Level>(entity.0)?.0 = level;
                    *world.get::<&mut DeathType>(entity.0)? = deathtype;
                    if let Ok(mut physical) =
                        world.get::<&mut Physical>(entity.0)
                    {
                        physical.damage = pdamage;
                        physical.defense = pdefense;
                    }
                    *world.get::<&mut Position>(entity.0)? = pos;
                    if let Ok(mut pvp) = world.get::<&mut PlayerPvP>(entity.0) {
                        pvp.pk = pk;
                        pvp.pvpon = pvpon;
                    }
                    world.get::<&mut SpriteImage>(entity.0)?.0 = sprite;
                    if let Ok(mut vital) = world.get::<&mut Vitals>(entity.0) {
                        vital.vital = vitals;
                        vital.vitalmax = vitalmax;
                    }
                }

                if did_spawn && content.game_content.finalized {
                    player_finalized(world, systems, &entity)?;
                }
            }
        }
    }

    Ok(())
}

pub fn handle_playermove(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let pos = data.read::<Position>()?;
        let _warp = data.read::<bool>()?;
        let _switch = data.read::<bool>()?;
        let dir = data.read::<u8>()?;

        if let Some(myentity) = content.game_content.myentity {
            if myentity != entity && world.contains(entity.0) {
                let player_pos = world.get_or_err::<Position>(&myentity)?;
                if is_map_connected(player_pos.map, pos.map) {
                    let mut movementbuffer =
                        world.get::<&mut MovementBuffer>(entity.0)?;
                    let movement_data = MovementData { end_pos: pos, dir };
                    if movementbuffer.data.is_empty() {
                        movementbuffer.data.push_back(movement_data);
                    } else if let Some(data) = movementbuffer.data.back() {
                        if *data != movement_data {
                            movementbuffer.data.push_back(movement_data);
                        }
                    }
                } else {
                    unload_player(world, systems, &entity)?;
                }
            }
        }
    }

    Ok(())
}

pub fn handle_playerwarp(
    socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let pos = data.read::<Position>()?;

        let old_pos = world.get_or_err::<Position>(&entity)?;

        if world.contains(entity.0) {
            *world.get::<&mut Position>(entity.0)? = pos;
            world.get::<&mut Movement>(entity.0)?.is_moving = false;
            world.get::<&mut PositionOffset>(entity.0)?.offset =
                Vec2::new(0.0, 0.0);

            let frame = world.get_or_err::<Dir>(&entity)?.0
                * PLAYER_SPRITE_FRAME_X as u8;
            set_player_frame(world, systems, &entity, frame as usize)?;
        }
        if let Some(myentity) = content.game_content.myentity {
            if myentity == entity {
                if old_pos.map != pos.map {
                    content.game_content.init_map(systems, pos.map);
                }
                update_camera(
                    world,
                    &mut content.game_content,
                    systems,
                    socket,
                )?;
                content.game_content.target.clear_target(socket, systems)?;
            }
        }
    }

    Ok(())
}

pub fn handle_playermapswap(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_dataremovelist(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let remove_list = data.read::<Vec<Entity>>()?;

    for entity in remove_list.iter() {
        let world_entity_type = world.get_or_default::<WorldEntityType>(entity);
        match world_entity_type {
            WorldEntityType::Player => {
                unload_player(world, systems, entity)?;
            }
            WorldEntityType::Npc => {
                unload_npc(world, systems, entity)?;
            }
            WorldEntityType::MapItem => {
                unload_mapitems(world, systems, entity)?;
            }
            _ => {}
        }
    }

    Ok(())
}

pub fn handle_dataremove(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_playerdir(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _dir = data.read::<u8>()?;

    Ok(())
}

pub fn handle_playervitals(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let mut vitals = [0; VITALS_MAX];
        vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let mut vitalmax = [0; VITALS_MAX];
        vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);

        if world.contains(entity.0) {
            if let Ok(mut vital) = world.get::<&mut Vitals>(entity.0) {
                vital.vital = vitals;
                vital.vitalmax = vitalmax;
            }

            let hpbar = world.get_or_err::<HPBar>(&entity)?;
            let mut size = systems.gfx.get_size(hpbar.bar_index);
            size.x = get_percent(vitals[0], vitalmax[0], 18) as f32;
            systems.gfx.set_size(hpbar.bar_index, size);
            if let Some(myentity) = content.game_content.myentity {
                if entity == myentity {
                    systems
                        .gfx
                        .set_visible(hpbar.bar_index, vitals[0] != vitalmax[0]);
                    systems
                        .gfx
                        .set_visible(hpbar.bg_index, vitals[0] != vitalmax[0]);

                    content.game_content.interface.vitalbar.update_bar_size(
                        systems,
                        0,
                        vitals[0],
                        vitalmax[0],
                    );
                    content.game_content.interface.vitalbar.update_bar_size(
                        systems,
                        1,
                        vitals[2],
                        vitalmax[2],
                    );
                }
            }
        }
    }

    Ok(())
}

pub fn handle_playerinv(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let items = data.read::<Vec<Item>>()?;

    if content.game_content.finalized {
        for (index, item) in items.iter().enumerate() {
            content
                .game_content
                .interface
                .inventory
                .update_inv_slot(systems, index, item);
        }
    }
    content.game_content.player_data.inventory = items;

    Ok(())
}

pub fn handle_playerinvslot(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let index = data.read::<usize>()?;
    let item = data.read::<Item>()?;

    content
        .game_content
        .interface
        .inventory
        .update_inv_slot(systems, index, &item);

    Ok(())
}

pub fn handle_playerstorage(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let items = data.read::<Vec<Item>>()?;

    if content.game_content.finalized {
        for (index, item) in items.iter().enumerate() {
            content
                .game_content
                .interface
                .storage
                .update_storage_slot(systems, index, item);
        }
    }
    content.game_content.player_data.storage = items;

    Ok(())
}

pub fn handle_playerstorageslot(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let index = data.read::<usize>()?;
    let item = data.read::<Item>()?;

    content
        .game_content
        .interface
        .storage
        .update_storage_slot(systems, index, &item);

    Ok(())
}

pub fn handle_keyinput(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_playerattack(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;

        if let Some(myentity) = content.game_content.myentity {
            if myentity != entity && world.contains(entity.0) {
                init_player_attack(world, systems, &entity, seconds)?
            }
        }
    }

    Ok(())
}

pub fn handle_playerequipment(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let entity = data.read::<Entity>()?;
    let equipment = data.read::<Equipment>()?;

    {
        *world.get::<&mut Equipment>(entity.0)? = equipment.clone();
    }

    if let Some(myentity) = content.game_content.myentity {
        if myentity == entity {
            for i in 0..MAX_EQPT {
                if content.game_content.player_data.equipment[i]
                    != equipment.items[i]
                {
                    content
                        .game_content
                        .interface
                        .profile
                        .update_equipment_slot(systems, i, &equipment.items[i]);
                    content.game_content.player_data.equipment[i] =
                        equipment.items[i];
                }
            }

            let damage = world
                .get_or_err::<Physical>(&myentity)?
                .damage
                .saturating_add(
                    player_get_weapon_damage(world, systems, &myentity)?.0
                        as u32,
                );
            content
                .game_content
                .interface
                .profile
                .set_profile_label_value(
                    systems,
                    ProfileLabel::Damage,
                    damage as u64,
                );
            let defense = world
                .get_or_err::<Physical>(&myentity)?
                .defense
                .saturating_add(
                    player_get_armor_defense(world, systems, &myentity)?.0
                        as u32,
                );
            content
                .game_content
                .interface
                .profile
                .set_profile_label_value(
                    systems,
                    ProfileLabel::Defense,
                    defense as u64,
                );
        }
    }

    Ok(())
}

pub fn handle_playeraction(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_playerlevel(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let level = data.read::<i32>()?;
    let levelexp = data.read::<u64>()?;

    content.game_content.player_data.levelexp = levelexp;

    if let Some(myentity) = content.game_content.myentity {
        if world.contains(myentity.0) {
            {
                world.get::<&mut Level>(myentity.0)?.0 = level;
            }
            let nextexp = player_get_next_lvl_exp(world, &myentity)?;

            if content.game_content.finalized {
                content.game_content.interface.vitalbar.update_bar_size(
                    systems,
                    2,
                    levelexp as i32,
                    nextexp as i32,
                );

                content
                    .game_content
                    .interface
                    .profile
                    .set_profile_label_value(
                        systems,
                        ProfileLabel::Level,
                        level as u64,
                    );
            }
        }
    }

    Ok(())
}

pub fn handle_playermoney(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _vals = data.read::<u64>()?;

    Ok(())
}

pub fn handle_playerstun(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_playervariables(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_playervariable(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_playerdeath(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_npcdeath(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_playerpvp(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_playerpk(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_playeremail(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_npcdata(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
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
        vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let mut vitalmax = [0; VITALS_MAX];
        vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let did_spawn = data.read::<bool>()?;

        if let Some(myentity) = content.game_content.myentity {
            if !world.contains(entity.0) {
                let client_map = world.get_or_err::<Position>(&myentity)?.map;
                let npc =
                    add_npc(world, systems, pos, client_map, Some(&entity))?;
                content.game_content.npcs.insert(npc);

                {
                    world.get::<&mut Dir>(entity.0)?.0 = dir;
                    world.get::<&mut Hidden>(entity.0)?.0 = hidden;
                    world.get::<&mut Level>(entity.0)?.0 = level;
                    *world.get::<&mut DeathType>(entity.0)? = deathtype;
                    *world.get::<&mut NpcMode>(entity.0)? = mode;
                    world.get::<&mut NpcIndex>(entity.0)?.0 = num;
                    if let Ok(mut physical) =
                        world.get::<&mut Physical>(entity.0)
                    {
                        physical.damage = pdamage;
                        physical.defense = pdefense;
                    }
                    *world.get::<&mut Position>(entity.0)? = pos;
                    world.get::<&mut SpriteImage>(entity.0)?.0 = sprite as u8;
                    if let Ok(mut vital) = world.get::<&mut Vitals>(entity.0) {
                        vital.vital = vitals;
                        vital.vitalmax = vitalmax;
                    }
                }

                if did_spawn && content.game_content.finalized {
                    npc_finalized(world, systems, &entity)?;
                }
            }
        }
    }

    Ok(())
}

pub fn handle_npcmove(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let pos = data.read::<Position>()?;
        let _warp = data.read::<bool>()?;
        let _switch = data.read::<bool>()?;
        let dir = data.read::<u8>()?;

        if world.contains(entity.0) {
            if let Some(myentity) = content.game_content.myentity {
                let player_pos = world.get_or_err::<Position>(&myentity)?;
                if is_map_connected(player_pos.map, pos.map) {
                    let mut movementbuffer =
                        world.get::<&mut MovementBuffer>(entity.0)?;
                    let movement_data = MovementData { end_pos: pos, dir };
                    if movementbuffer.data.is_empty() {
                        movementbuffer.data.push_back(movement_data);
                    } else if let Some(data) = movementbuffer.data.back() {
                        if *data != movement_data {
                            movementbuffer.data.push_back(movement_data);
                        }
                    }
                } else {
                    unload_npc(world, systems, &entity)?;
                }
            }
        }
    }

    Ok(())
}

pub fn handle_npcwarp(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let pos = data.read::<Position>()?;

        if world.contains(entity.0) {
            *world.get::<&mut Position>(entity.0)? = pos;
            world.get::<&mut Movement>(entity.0)?.is_moving = false;
            world.get::<&mut PositionOffset>(entity.0)?.offset =
                Vec2::new(0.0, 0.0);
            let frame =
                world.get_or_err::<Dir>(&entity)?.0 * NPC_SPRITE_FRAME_X as u8;
            set_npc_frame(world, systems, &entity, frame as usize)?;
        }
    }

    Ok(())
}

pub fn handle_npcdir(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let dir = data.read::<u8>()?;

        if world.contains(entity.0) {
            world.get::<&mut Dir>(entity.0)?.0 = dir;
            let frame =
                world.get_or_err::<Dir>(&entity)?.0 * NPC_SPRITE_FRAME_X as u8;
            set_npc_frame(world, systems, &entity, frame as usize)?;
        }
    }

    Ok(())
}

pub fn handle_npcvital(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;
        let mut vitals = [0; VITALS_MAX];
        vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let mut vitalmax = [0; VITALS_MAX];
        vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);

        if world.contains(entity.0) {
            if let Ok(mut vital) = world.get::<&mut Vitals>(entity.0) {
                vital.vital = vitals;
                vital.vitalmax = vitalmax;
            }

            let bar_index = world.get_or_err::<HPBar>(&entity)?.bar_index;
            let mut size = systems.gfx.get_size(bar_index);
            size.x = get_percent(vitals[0], vitalmax[0], 18) as f32;
            systems.gfx.set_size(bar_index, size);
        }
    }

    Ok(())
}

pub fn handle_npcattack(
    _socket: &mut Socket,
    world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;

        if world.contains(entity.0) {
            //ToDo Npc Attack
        }
    }

    Ok(())
}

pub fn handle_npcstun(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_chatmsg(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let _channel = data.read::<MessageChannel>()?;
        let head_string = data.read::<String>()?;
        let msg_string = data.read::<String>()?;
        let _useraccess = data.read::<Option<UserAccess>>()?;

        let header = if !head_string.is_empty() {
            Some((head_string, COLOR_GREEN))
        } else {
            None
        };

        buffer
            .chatbuffer
            .add_task(ChatTask::new((msg_string, COLOR_WHITE), header));
    }

    Ok(())
}

pub fn handle_sound(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_target(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_synccheck(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_entityunload(
    socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<Entity>()?;

        if world.contains(entity.0) {
            if let Some(target_entity) = content.game_content.target.entity {
                if target_entity == entity {
                    content
                        .game_content
                        .target
                        .clear_target(socket, systems)?;
                }
            }

            let world_entity_type =
                world.get_or_default::<WorldEntityType>(&entity);
            match world_entity_type {
                WorldEntityType::Player => {
                    unload_player(world, systems, &entity)?;
                }
                WorldEntityType::Npc => {
                    unload_npc(world, systems, &entity)?;
                }
                WorldEntityType::MapItem => {
                    unload_mapitems(world, systems, &entity)?;
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
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_openstorage(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _ = data.read::<u32>()?;

    open_interface(
        &mut content.game_content.interface,
        systems,
        Window::Storage,
    );

    content.game_content.player_data.is_using_type = IsUsingType::Bank;

    Ok(())
}

pub fn handle_openshop(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let shop_index = data.read::<u16>()?;

    open_interface(&mut content.game_content.interface, systems, Window::Shop);
    content
        .game_content
        .interface
        .shop
        .set_shop(systems, shop_index as usize);

    content.game_content.player_data.is_using_type =
        IsUsingType::Store(shop_index as i64);

    Ok(())
}

pub fn handle_clearisusingtype(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _ = data.read::<u16>()?;

    match content.game_content.player_data.is_using_type {
        IsUsingType::Bank => close_interface(
            &mut content.game_content.interface,
            systems,
            Window::Storage,
        ),
        IsUsingType::Store(_) => close_interface(
            &mut content.game_content.interface,
            systems,
            Window::Shop,
        ),
        IsUsingType::Trading(_) => {
            close_interface(
                &mut content.game_content.interface,
                systems,
                Window::Trade,
            );
            content
                .game_content
                .interface
                .trade
                .clear_trade_items(systems);
        }
        _ => {}
    }

    content.game_content.player_data.is_using_type = IsUsingType::None;

    Ok(())
}

pub fn handle_updatetradeitem(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let same_entity = data.read::<bool>()?;
    let trade_slot = data.read::<u16>()?;
    let item = data.read::<Item>()?;

    content.game_content.interface.trade.update_trade_slot(
        systems,
        trade_slot as usize,
        &item,
        same_entity,
    );

    Ok(())
}

pub fn handle_updatetrademoney(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let amount = data.read::<u64>()?;

    content
        .game_content
        .interface
        .trade
        .update_trade_money(systems, amount);

    Ok(())
}

pub fn handle_inittrade(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let target_entity = data.read::<Entity>()?;

    content.game_content.player_data.is_using_type =
        IsUsingType::Trading(target_entity);

    open_interface(&mut content.game_content.interface, systems, Window::Trade);
    content
        .game_content
        .interface
        .trade
        .clear_trade_items(systems);

    Ok(())
}

pub fn handle_tradestatus(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let my_status = data.read::<TradeStatus>()?;
    let their_status = data.read::<TradeStatus>()?;

    content.game_content.interface.trade.trade_status = my_status;

    if my_status == TradeStatus::Accepted
        && their_status == TradeStatus::Accepted
    {
        content.game_content.interface.trade.button[1]
            .change_text(systems, "Confirm".into());
        content.game_content.interface.trade.update_status(
            systems,
            "Click the 'Confirm' Button to proceed".into(),
        );
    }

    match my_status {
        TradeStatus::None => {
            content
                .game_content
                .interface
                .trade
                .update_my_status(systems, "My Trade: Preparing...".into());
        }
        TradeStatus::Accepted => {
            content
                .game_content
                .interface
                .trade
                .update_my_status(systems, "My Trade: Submitted".into());
        }
        TradeStatus::Submitted => {
            content
                .game_content
                .interface
                .trade
                .update_my_status(systems, "My Trade: Confirmed".into());
        }
    }
    match their_status {
        TradeStatus::None => {
            content.game_content.interface.trade.update_their_status(
                systems,
                "Their Trade: Preparing...".into(),
            );
        }
        TradeStatus::Accepted => {
            content
                .game_content
                .interface
                .trade
                .update_their_status(systems, "Their Trade: Submitted".into());
        }
        TradeStatus::Submitted => {
            content
                .game_content
                .interface
                .trade
                .update_their_status(systems, "Their Trade: Confirmed".into());
        }
    }

    Ok(())
}

pub fn handle_traderequest(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let entity = data.read::<Entity>()?;
    if !world.contains(entity.0) {
        return Ok(());
    }

    let name = world.cloned_get_or_err::<EntityName>(&entity)?.0;

    alert.show_alert(
        systems,
        AlertType::Confirm,
        "Would you like to accept this trade request?".into(),
        format!("{name} would like to trade with you"),
        250,
        AlertIndex::TradeRequest,
        false,
    );

    Ok(())
}

pub fn handle_playitemsfx(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut ByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let index = data.read::<u16>()?;

    if let Some(sfx_name) = &systems.base.item[index as usize].sound_index {
        let volume = systems.config.sfx_volume as f32 * 0.01;
        systems
            .audio
            .play_effect(&format!("./audio/{}", sfx_name), volume)?;
    }

    Ok(())
}
