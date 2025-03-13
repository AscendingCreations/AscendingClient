use std::ops::Range;

use crate::{
    Alert, AlertIndex, AlertType, BufferTask, ChatTask, Content, DeathType,
    EncryptionState, Entity, EntityKind, Equipment, FtlType, GlobalKey,
    IsUsingType, Item, MAX_EQPT, MapItem, MessageChannel, MovementData,
    NPC_SPRITE_FRAME_X, NpcMode, Position, ProfileLabel, Result, Socket,
    SystemHolder, TradeStatus, UserAccess, VITALS_MAX, Window, World,
    add_float_text, add_npc, close_interface,
    content::game_content::{interface::chatbox::*, player::*},
    create_npc_light,
    data_types::*,
    dir_to_enum,
    fade::*,
    finalize_entity, get_percent, get_start_map_pos, init_npc_attack,
    is_map_connected, npc_finalized, open_interface, player_get_armor_defense,
    player_get_weapon_damage, send_handshake, set_npc_frame, unload_mapitems,
    unload_npc, update_camera, update_mapitem_position, update_npc_camera,
    update_player,
};
use graphics::*;

use log::info;
use mmap_bytey::MByteBuffer;

pub fn handle_ping(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    _data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let end_time = MyInstant::now();

    let elapse_time =
        end_time.duration_since(content.ping_start.0).as_millis() as u64;

    if systems.config.show_average_ping {
        let count =
            content.game_content.interface.average_ping_collection.len();
        if count > 0 {
            let sum: u64 = content
                .game_content
                .interface
                .average_ping_collection
                .iter()
                .sum();
            if sum > 0 {
                let average = sum / count as u64;
                systems.gfx.set_text(
                    &mut systems.renderer,
                    &content.game_content.interface.average_ping,
                    &format!("Av. Ping: {:?}", average),
                );
            }
            if count >= 20 {
                content
                    .game_content
                    .interface
                    .average_ping_collection
                    .pop_back();
            }
        }
        content
            .game_content
            .interface
            .average_ping_collection
            .push_front(elapse_time);
    }

    systems.gfx.set_text(
        &mut systems.renderer,
        &content.game_content.interface.ping_text,
        &format!("Ping: {:?}", elapse_time),
    );

    Ok(())
}

pub fn handle_alertmsg(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let message = data.read::<String>()?;
    let _close = data.read::<u8>()?;

    alert.show_alert(
        systems,
        AlertType::Inform,
        message,
        "Alert Message".into(),
        250,
        AlertIndex::None,
        false,
    );

    Ok(())
}

pub fn handle_fltalert(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _flttype = data.read::<FtlType>()?;
    let _message = data.read::<String>()?;

    Ok(())
}

pub fn handle_handshake(
    socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let code = data.read::<String>()?;
    let handshake = data.read::<String>()?;
    systems.config.reconnect_code = code;
    systems.config.save_config("settings.toml");
    socket.encrypt_state = EncryptionState::None;
    socket.client.socket.set_nodelay(true)?;
    send_handshake(socket, handshake)
}

pub fn handle_loginok(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
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

pub fn handle_mapitems(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;
        let pos = data.read::<Position>()?;
        let item = data.read::<Item>()?;
        let _owner = data.read::<Option<GlobalKey>>()?;
        let _did_spawn = data.read::<bool>()?;

        if let Some(myentity) = content.game_content.myentity {
            if !world.entities.contains_key(entity) {
                let client_pos = if let Some(Entity::Player(p_data)) =
                    world.entities.get(myentity)
                {
                    p_data.pos
                } else {
                    Position::default()
                };

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
                    client_pos.map,
                    Some(entity),
                )?;

                content.game_content.mapitems.borrow_mut().insert(mapitem);

                if content.game_content.finalized {
                    MapItem::finalized(world, systems, entity)?;
                    if let Some(Entity::MapItem(mi_data)) =
                        world.entities.get(entity)
                    {
                        update_mapitem_position(
                            systems,
                            &content.game_content,
                            mi_data.sprite_index,
                            &pos,
                            mi_data.pos_offset,
                            mi_data.light,
                        );
                    }
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
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let entity = data.read::<GlobalKey>()?;
    content.game_content.myentity = Some(entity);
    Ok(())
}

pub fn handle_move_ok(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    // TODO use this to reset players position, reload stuff etc.
    let _move_ok = data.read::<bool>()?;
    content.game_content.can_move = true;
    //info!("move allowed: {move_ok}");
    //content.game_content.myentity = Some(entity);
    Ok(())
}

pub fn handle_playerdata(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    if let Some(entity) = content.game_content.myentity {
        let username = data.read::<String>()?;
        let useraccess = data.read::<UserAccess>()?;
        let dir = data.read::<u8>()?;
        let equipment = data.read::<Equipment>()?;
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

        if !world.entities.contains_key(entity) || !content.game_content.in_game
        {
            let player = add_player(
                world,
                systems,
                pos,
                pos.map,
                Some(entity),
                sprite as usize,
            )?;
            // Create Lights
            create_player_light(
                world,
                systems,
                &content.game_content.game_lights,
                entity,
            );

            content.game_content.players.borrow_mut().insert(player);
            content.game_content.in_game = true;
        }

        content.game_content.player_data.equipment[..]
            .copy_from_slice(&equipment.items);

        if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
            systems.gfx.set_text(
                &mut systems.renderer,
                &p_data.name_map.0,
                &username,
            );
            p_data.hp_bar.visible = vitals[0] != vitalmax[0];
            p_data.entity_name.0.clone_from(&username);
            p_data.user_access = useraccess;
            p_data.dir = dir;
            p_data.equipment = equipment;
            p_data.level = level;
            p_data.death_type = deathtype;
            p_data.physical.damage = pdamage;
            p_data.physical.defense = pdefense;
            p_data.pos = pos;
            p_data.pvp.pk = pk;
            p_data.pvp.pvpon = pvpon;
            p_data.sprite.0 = sprite;
            p_data.vitals.vital = vitals;
            p_data.vitals.vitalmax = vitalmax;
        }
    }
    Ok(())
}

pub fn handle_playerspawn(
    socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let username = data.read::<String>()?;
        let dir = data.read::<u8>()?;
        let entity = data.read::<GlobalKey>()?;
        let level = data.read::<i32>()?;
        let deathtype = data.read::<DeathType>()?;
        let pdamage = data.read::<u32>()?;
        let pdefense = data.read::<u32>()?;
        let pos = data.read::<Position>()?;
        let sprite = data.read::<u16>()?;
        let mut vitals = [0; VITALS_MAX];
        vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let mut vitalmax = [0; VITALS_MAX];
        vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let useraccess = data.read::<UserAccess>()?;
        let equipment = data.read::<Equipment>()?;
        let pk = data.read::<bool>()?;
        let pvpon = data.read::<bool>()?;
        let _did_spawn = data.read::<bool>()?;

        if let Some(myentity) = content.game_content.myentity {
            if myentity != entity && !world.entities.contains_key(entity) {
                let client_pos = if let Some(Entity::Player(p_data)) =
                    world.entities.get(myentity)
                {
                    p_data.pos
                } else {
                    Position::default()
                };

                let player = add_player(
                    world,
                    systems,
                    pos,
                    client_pos.map,
                    Some(entity),
                    sprite as usize,
                )?;
                create_player_light(
                    world,
                    systems,
                    &content.game_content.game_lights,
                    entity,
                );

                content.game_content.players.borrow_mut().insert(player);

                if let Some(Entity::Player(p_data)) =
                    world.entities.get_mut(entity)
                {
                    systems.gfx.set_text(
                        &mut systems.renderer,
                        &p_data.name_map.0,
                        &username,
                    );
                    p_data.entity_name.0.clone_from(&username);
                    p_data.user_access = useraccess;
                    p_data.dir = dir;
                    p_data.equipment = equipment;
                    p_data.level = level;
                    p_data.death_type = deathtype;
                    p_data.physical.damage = pdamage;
                    p_data.physical.defense = pdefense;
                    p_data.pos = pos;
                    p_data.pvp.pk = pk;
                    p_data.pvp.pvpon = pvpon;
                    p_data.sprite.0 = sprite as u8;
                    p_data.vitals.vital = vitals;
                    p_data.vitals.vitalmax = vitalmax;
                }

                if content.game_content.finalized {
                    player_finalized(world, systems, entity)?;
                    update_player_camera(
                        world,
                        systems,
                        socket,
                        entity,
                        &mut content.game_content,
                    )?;
                }
            }
        }
    }

    Ok(())
}

pub fn handle_move(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;
        let pos = data.read::<Position>()?;
        let _warp = data.read::<bool>()?;
        let _switch = data.read::<bool>()?;
        let dir = data.read::<u8>()?;

        if let Some(myentity) = content.game_content.myentity {
            if myentity != entity && world.entities.contains_key(entity) {
                let player_pos = if let Some(Entity::Player(p_data)) =
                    world.entities.get(myentity)
                {
                    p_data.pos
                } else {
                    Position::default()
                };

                if is_map_connected(player_pos.map, pos.map) {
                    if let Some(entity_data) = world.entities.get_mut(entity) {
                        let movement_data = MovementData { end_pos: pos, dir };

                        match entity_data {
                            Entity::Player(p_data) => {
                                if p_data.movement_buffer.is_empty() {
                                    p_data
                                        .movement_buffer
                                        .push_back(movement_data);
                                } else if let Some(data) =
                                    p_data.movement_buffer.back()
                                {
                                    if *data != movement_data {
                                        p_data
                                            .movement_buffer
                                            .push_back(movement_data);
                                    }
                                }
                            }
                            Entity::Npc(n_data) => {
                                if n_data.movement_buffer.is_empty() {
                                    n_data
                                        .movement_buffer
                                        .push_back(movement_data);
                                } else if let Some(data) =
                                    n_data.movement_buffer.back()
                                {
                                    if *data != movement_data {
                                        n_data
                                            .movement_buffer
                                            .push_back(movement_data);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                } else {
                    let entity_kind = world.get_kind(entity)?;

                    match entity_kind {
                        EntityKind::Player => {
                            unload_player(
                                world,
                                systems,
                                &content.game_content,
                                entity,
                            )?;
                            content
                                .game_content
                                .players
                                .borrow_mut()
                                .swap_remove(&entity);
                        }
                        EntityKind::Npc => {
                            unload_npc(
                                world,
                                systems,
                                &content.game_content,
                                entity,
                            )?;
                            content
                                .game_content
                                .npcs
                                .borrow_mut()
                                .swap_remove(&entity);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn handle_warp(
    socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;
        let pos = data.read::<Position>()?;

        if !world.entities.contains_key(entity) {
            continue;
        }

        let (old_pos, dir) =
            if let Some(entity_data) = world.entities.get_mut(entity) {
                match entity_data {
                    Entity::Player(p_data) => {
                        let old_pos = p_data.pos;
                        p_data.movement_buffer.clear();
                        p_data.movement.is_moving = false;
                        p_data.pos = pos;
                        p_data.pos_offset = Vec2::new(0.0, 0.0);

                        (old_pos, p_data.dir)
                    }
                    Entity::Npc(n_data) => {
                        let old_pos = n_data.pos;
                        n_data.movement_buffer.clear();
                        n_data.movement.is_moving = false;
                        n_data.pos = pos;
                        n_data.pos_offset = Vec2::new(0.0, 0.0);

                        (old_pos, n_data.dir)
                    }
                    _ => {
                        continue;
                    }
                }
            } else {
                continue;
            };

        let world_entity_type = world.get_kind(entity)?;

        if world_entity_type == EntityKind::Player {
            let frame = dir * PLAYER_SPRITE_FRAME_X as u8;
            set_player_frame(world, systems, entity, frame as usize)?;
        } else if world_entity_type == EntityKind::Npc {
            let frame = dir * NPC_SPRITE_FRAME_X as u8;
            set_npc_frame(world, systems, entity, frame as usize)?;
        }

        if world_entity_type == EntityKind::Player {
            if let Some(myentity) = content.game_content.myentity {
                let client_pos = if let Some(Entity::Player(p_data)) =
                    world.entities.get(myentity)
                {
                    p_data.pos
                } else {
                    Position::default()
                };

                if myentity == entity {
                    socket.client.sends.clear();

                    if old_pos.map != pos.map {
                        content
                            .game_content
                            .init_map(systems, pos.map, buffer)?;
                        finalize_entity(world, systems)?;
                        content.game_content.refresh_map = true;
                    }
                    content.game_content.can_move = true;

                    if systems.map_fade.f_alpha > 0 {
                        systems.map_fade.init_fade(
                            &mut systems.gfx,
                            FadeType::Out,
                            0,
                            FadeData::None,
                        );
                    }

                    update_camera(
                        world,
                        &mut content.game_content,
                        systems,
                        socket,
                    )?;
                    if let Some(target_entity) =
                        content.game_content.target.entity
                    {
                        if let Some(entity_data) =
                            world.entities.get_mut(target_entity)
                        {
                            match entity_data {
                                Entity::Player(p_data) => {
                                    systems
                                        .gfx
                                        .set_visible(&p_data.name_map.0, false);
                                    systems
                                        .gfx
                                        .set_visible(&p_data.name_map.0, false);

                                    content.game_content.target.clear_target(
                                        socket,
                                        systems,
                                        &mut p_data.hp_bar,
                                    )?;
                                }
                                Entity::Npc(n_data) => {
                                    systems
                                        .gfx
                                        .set_visible(&n_data.name_map.0, false);
                                    systems
                                        .gfx
                                        .set_visible(&n_data.name_map.0, false);

                                    content.game_content.target.clear_target(
                                        socket,
                                        systems,
                                        &mut n_data.hp_bar,
                                    )?;
                                }
                                _ => {}
                            }
                        }
                    }
                } else if !is_map_connected(client_pos.map, pos.map) {
                    if let Some(target_entity) =
                        content.game_content.target.entity
                    {
                        if target_entity == entity {
                            if let Some(entity_data) =
                                world.entities.get_mut(target_entity)
                            {
                                match entity_data {
                                    Entity::Player(p_data) => {
                                        systems.gfx.set_visible(
                                            &p_data.name_map.0,
                                            false,
                                        );
                                        systems.gfx.set_visible(
                                            &p_data.name_map.0,
                                            false,
                                        );

                                        content
                                            .game_content
                                            .target
                                            .clear_target(
                                                socket,
                                                systems,
                                                &mut p_data.hp_bar,
                                            )?;
                                    }
                                    Entity::Npc(n_data) => {
                                        systems.gfx.set_visible(
                                            &n_data.name_map.0,
                                            false,
                                        );
                                        systems.gfx.set_visible(
                                            &n_data.name_map.0,
                                            false,
                                        );

                                        content
                                            .game_content
                                            .target
                                            .clear_target(
                                                socket,
                                                systems,
                                                &mut n_data.hp_bar,
                                            )?;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    unload_player(
                        world,
                        systems,
                        &content.game_content,
                        entity,
                    )?;
                    content
                        .game_content
                        .players
                        .borrow_mut()
                        .swap_remove(&entity);
                } else {
                    update_player_camera(
                        world,
                        systems,
                        socket,
                        entity,
                        &mut content.game_content,
                    )?;
                }
            }
        }
    }

    Ok(())
}

pub fn handle_dir(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;
        let dir = data.read::<u8>()?;

        if world.entities.contains_key(entity) {
            let is_moving =
                if let Some(entity_data) = world.entities.get_mut(entity) {
                    match entity_data {
                        Entity::Player(p_data) => {
                            p_data.dir = dir;

                            p_data.movement.is_moving
                        }
                        Entity::Npc(n_data) => {
                            n_data.dir = dir;

                            n_data.movement.is_moving
                        }
                        _ => {
                            continue;
                        }
                    }
                } else {
                    continue;
                };

            if !is_moving {
                let entity_kind = world.get_kind(entity)?;

                if entity_kind == EntityKind::Player {
                    let frame = dir * PLAYER_SPRITE_FRAME_X as u8;
                    set_player_frame(world, systems, entity, frame as usize)?;
                } else if entity_kind == EntityKind::Npc {
                    let frame = dir * NPC_SPRITE_FRAME_X as u8;
                    set_npc_frame(world, systems, entity, frame as usize)?;
                };
            }
        }
    }

    Ok(())
}

pub fn handle_vitals(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;
        let mut vitals = [0; VITALS_MAX];
        vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let mut vitalmax = [0; VITALS_MAX];
        vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);

        if world.entities.contains_key(entity) {
            let hpbar =
                if let Some(entity_data) = world.entities.get_mut(entity) {
                    match entity_data {
                        Entity::Player(p_data) => {
                            p_data.vitals.vital = vitals;
                            p_data.vitals.vitalmax = vitalmax;

                            p_data.hp_bar
                        }
                        Entity::Npc(n_data) => {
                            n_data.vitals.vital = vitals;
                            n_data.vitals.vitalmax = vitalmax;

                            n_data.hp_bar
                        }
                        _ => {
                            continue;
                        }
                    }
                } else {
                    continue;
                };

            let mut size = systems.gfx.get_size(&hpbar.bar_index);
            size.x = get_percent(vitals[0], vitalmax[0], 18) as f32;
            systems.gfx.set_size(&hpbar.bar_index, size);

            let entity_kind = world.get_kind(entity)?;

            if entity_kind == EntityKind::Player {
                if let Some(myentity) = content.game_content.myentity {
                    if entity == myentity {
                        if let Some(Entity::Player(p_data)) =
                            world.entities.get_mut(entity)
                        {
                            p_data.hp_bar.visible = vitals[0] != vitalmax[0];
                        }

                        systems.gfx.set_visible(
                            &hpbar.bar_index,
                            vitals[0] != vitalmax[0],
                        );
                        systems.gfx.set_visible(
                            &hpbar.bg_index,
                            vitals[0] != vitalmax[0],
                        );

                        content
                            .game_content
                            .interface
                            .vitalbar
                            .update_bar_size(
                                systems,
                                0,
                                vitals[0],
                                vitalmax[0],
                            );
                        content
                            .game_content
                            .interface
                            .vitalbar
                            .update_bar_size(
                                systems,
                                1,
                                vitals[2],
                                vitalmax[2],
                            );
                    }
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
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let items = data.read::<Vec<Item>>()?;

    content.game_content.player_data.inventory[..].copy_from_slice(&items);

    if content.game_content.finalized {
        for (index, item) in items.iter().enumerate() {
            content
                .game_content
                .interface
                .inventory
                .update_inv_slot(systems, index, item);
        }
    }

    Ok(())
}

pub fn handle_playerinvslot(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
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
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let start = data.read::<usize>()?;
    let end = data.read::<usize>()?;
    let items = data.read::<Vec<Item>>()?;

    content.game_content.player_data.storage[start..end]
        .copy_from_slice(&items);
    if content.game_content.finalized {
        for (index, item) in content.game_content.player_data.storage
            [start..end]
            .iter()
            .enumerate()
        {
            content.game_content.interface.storage.update_storage_slot(
                systems,
                index + start,
                item,
            );
        }
    }

    Ok(())
}

pub fn handle_playerstorageslot(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
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

pub fn handle_attack(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;

        if world.entities.contains_key(entity) {
            let entity_kind = world.get_kind(entity)?;

            match entity_kind {
                EntityKind::Player => {
                    if let Some(myentity) = content.game_content.myentity {
                        if myentity != entity {
                            init_player_attack(world, systems, entity, seconds)?
                        }
                    }
                }
                EntityKind::Npc => {
                    init_npc_attack(world, systems, entity, seconds)?;
                }
                _ => {}
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
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let entity = data.read::<GlobalKey>()?;
    let equipment = data.read::<Equipment>()?;

    let (b_damage, b_defense) =
        if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
            p_data.equipment.clone_from(&equipment);

            (p_data.physical.damage, p_data.physical.defense)
        } else {
            return Ok(());
        };

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

            let damage = b_damage.saturating_add(
                player_get_weapon_damage(world, systems, myentity)?.0 as u32,
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
            let defense = b_defense.saturating_add(
                player_get_armor_defense(world, systems, myentity)?.0 as u32,
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

pub fn handle_playerlevel(
    _socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let level = data.read::<i32>()?;
    let levelexp = data.read::<u64>()?;

    content.game_content.player_data.levelexp = levelexp;

    if let Some(myentity) = content.game_content.myentity {
        if world.entities.contains_key(myentity) {
            if let Some(Entity::Player(p_data)) =
                world.entities.get_mut(myentity)
            {
                p_data.level = level;
            }

            let nextexp = player_get_next_lvl_exp(world, myentity)?;

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
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let vals = data.read::<u64>()?;

    content.game_content.player_data.player_money = vals;
    content
        .game_content
        .interface
        .profile
        .set_profile_label_value(systems, ProfileLabel::Money, vals);

    Ok(())
}

pub fn handle_death(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let _entity = data.read::<GlobalKey>()?;
        let _deathtype = data.read::<DeathType>()?;
    }

    Ok(())
}

pub fn handle_playerpk(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    _data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    Ok(())
}

pub fn handle_npcdata(
    socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let dir = data.read::<u8>()?;
        let entity = data.read::<GlobalKey>()?;
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
        let _did_spawn = data.read::<bool>()?;

        if let Some(myentity) = content.game_content.myentity {
            if !world.entities.contains_key(entity) {
                let client_pos = if let Some(Entity::Player(p_data)) =
                    world.entities.get(myentity)
                {
                    p_data.pos
                } else {
                    Position::default()
                };

                let npc = add_npc(
                    world,
                    systems,
                    pos,
                    client_pos.map,
                    Some(entity),
                    num as usize,
                )?;
                create_npc_light(
                    world,
                    systems,
                    &content.game_content.game_lights,
                    entity,
                );

                content.game_content.npcs.borrow_mut().insert(npc);

                if let Some(Entity::Npc(n_data)) =
                    world.entities.get_mut(entity)
                {
                    if let Some(npc_data) = systems.base.npc.get(num as usize) {
                        systems.gfx.set_text(
                            &mut systems.renderer,
                            &n_data.name_map.0,
                            &npc_data.name,
                        );
                    }

                    n_data.dir = dir;
                    n_data.level = level;
                    n_data.death_type = deathtype;
                    n_data.mode = mode;
                    n_data.entity_index = num;
                    n_data.pos = pos;
                    n_data.physical.damage = pdamage;
                    n_data.physical.defense = pdefense;
                    n_data.sprite.0 = sprite as u8;
                    n_data.vitals.vital = vitals;
                    n_data.vitals.vitalmax = vitalmax;
                }

                if content.game_content.finalized {
                    npc_finalized(world, systems, entity)?;

                    update_npc_camera(
                        world,
                        systems,
                        entity,
                        socket,
                        &mut content.game_content,
                    )?;
                }
            }
        }
    }

    Ok(())
}

pub fn handle_chatmsg(
    _socket: &mut Socket,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let channel = data.read::<MessageChannel>()?;
        let head_string = data.read::<String>()?;
        let msg_string = data.read::<String>()?;
        let _useraccess = data.read::<Option<UserAccess>>()?;

        let header = if !head_string.is_empty() {
            let color = match channel {
                MessageChannel::Global => COLOR_GREEN,
                MessageChannel::Map => COLOR_BLUE,
                MessageChannel::Private => COLOR_RED,
                _ => COLOR_WHITE,
            };
            Some((head_string, color))
        } else {
            None
        };

        buffer.chatbuffer.add_task(ChatTask::new(
            (msg_string, COLOR_WHITE),
            header,
            channel,
        ));
    }

    Ok(())
}

pub fn handle_entityunload(
    socket: &mut Socket,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;

        if world.entities.contains_key(entity) {
            if let Some(target_entity) = content.game_content.target.entity {
                if target_entity == entity {
                    if let Some(entity_data) =
                        world.entities.get_mut(target_entity)
                    {
                        match entity_data {
                            Entity::Player(p_data) => {
                                content.game_content.target.clear_target(
                                    socket,
                                    systems,
                                    &mut p_data.hp_bar,
                                )?;
                            }
                            Entity::Npc(n_data) => {
                                content.game_content.target.clear_target(
                                    socket,
                                    systems,
                                    &mut n_data.hp_bar,
                                )?;
                            }
                            _ => {}
                        }
                    }
                }
            }

            let world_entity_type = world.get_kind(entity)?;
            match world_entity_type {
                EntityKind::Player => {
                    unload_player(
                        world,
                        systems,
                        &content.game_content,
                        entity,
                    )?;
                    content
                        .game_content
                        .players
                        .borrow_mut()
                        .swap_remove(&entity);
                }
                EntityKind::Npc => {
                    unload_npc(world, systems, &content.game_content, entity)?;
                    content.game_content.npcs.borrow_mut().swap_remove(&entity);
                }
                EntityKind::MapItem => {
                    unload_mapitems(
                        world,
                        systems,
                        &content.game_content,
                        entity,
                    )?;
                    content
                        .game_content
                        .mapitems
                        .borrow_mut()
                        .swap_remove(&entity);
                }
                _ => {}
            }
        }
    }

    Ok(())
}

pub fn handle_openstorage(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _ = data.read::<u32>()?;

    open_interface(
        &mut content.game_content.interface,
        systems,
        Window::Storage,
    );

    content
        .game_content
        .keyinput
        .iter_mut()
        .for_each(|key_press| *key_press = false);

    content.game_content.player_data.is_using_type = IsUsingType::Bank;

    Ok(())
}

pub fn handle_openshop(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
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

    content
        .game_content
        .keyinput
        .iter_mut()
        .for_each(|key_press| *key_press = false);

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
    data: &mut MByteBuffer,
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
    data: &mut MByteBuffer,
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
    data: &mut MByteBuffer,
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
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let target_entity = data.read::<GlobalKey>()?;

    content.game_content.player_data.is_using_type =
        IsUsingType::Trading(target_entity);

    open_interface(&mut content.game_content.interface, systems, Window::Trade);
    content
        .game_content
        .interface
        .trade
        .clear_trade_items(systems);

    content
        .game_content
        .keyinput
        .iter_mut()
        .for_each(|key_press| *key_press = false);

    Ok(())
}

pub fn handle_tradestatus(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
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
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let entity = data.read::<GlobalKey>()?;
    if !world.entities.contains_key(entity) {
        return Ok(());
    }

    let name = if let Some(Entity::Player(p_data)) = world.entities.get(entity)
    {
        p_data.entity_name.0.clone()
    } else {
        return Ok(());
    };

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
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let index = data.read::<u16>()?;

    if let Some(sfx_name) = &systems.base.item[index as usize].sound_index {
        let volume = systems.config.sfx_volume as f32 * 0.01;
        systems
            .audio
            .play_effect(format!("./audio/{}", sfx_name), volume)?;
    }

    Ok(())
}

pub fn handle_damage(
    _socket: &mut Socket,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let _entity = data.read::<GlobalKey>()?;
        let amount = data.read::<u16>()?;
        let pos = data.read::<Position>()?;
        let is_damage = data.read::<bool>()?;

        let (text, color) = if is_damage {
            (format!("-{}", amount), COLOR_RED)
        } else {
            (format!("+{}", amount), COLOR_GREEN)
        };

        add_float_text(systems, &mut content.game_content, pos, text, color);
    }

    Ok(())
}
