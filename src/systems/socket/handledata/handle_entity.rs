use graphics::*;
use mmap_bytey::MByteBuffer;

use crate::{
    Alert, DeathType, Entity, EntityKind, Equipment, GlobalKey, Item,
    MovementData, NpcMode, Position, Result, UserAccess, VITALS_MAX, World,
    content::{
        Content, MapItem, NPC_SPRITE_FRAME_X, PLAYER_SPRITE_FRAME_X, add_npc,
        add_player, create_npc_light, create_player_light, finalize_entity,
        init_npc_attack, init_player_attack, is_map_connected, npc_finalized,
        player_finalized, set_npc_frame, set_player_frame, unload_mapitems,
        unload_npc, unload_player, update_mapitem_position, update_npc_camera,
        update_player_camera,
    },
    systems::{
        BufferTask, FadeData, FadeType, Poller, SystemHolder, get_percent,
        mapper::PacketPasser,
    },
};

pub fn handle_playerspawn(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
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

        if let Some(myentity) = passer.content.game_content.myentity
            && myentity != entity
            && !passer.world.entities.contains_key(entity)
        {
            let client_pos = if let Some(Entity::Player(p_data)) =
                passer.world.entities.get(myentity)
            {
                p_data.pos
            } else {
                Position::default()
            };

            let player = add_player(
                passer.world,
                passer.systems,
                pos,
                client_pos.map,
                entity,
                sprite as usize,
            )?;
            create_player_light(
                passer.world,
                passer.systems,
                &passer.content.game_content.game_lights,
                entity,
            );

            passer
                .content
                .game_content
                .players
                .borrow_mut()
                .insert(player);

            if let Some(Entity::Player(p_data)) =
                passer.world.entities.get_mut(entity)
            {
                passer.systems.gfx.set_text(
                    &mut passer.systems.renderer,
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

            if passer.content.game_content.finalized {
                player_finalized(
                    passer.world,
                    passer.systems,
                    entity,
                    passer.content.game_content.map.map_pos,
                    passer.content.game_content.game_lights,
                    false,
                )?;
                update_player_camera(
                    passer.world,
                    passer.systems,
                    passer.socket,
                    entity,
                    &mut passer.content.game_content,
                )?;
            }
        }
    }

    Ok(())
}

pub fn handle_npcdata(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
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

        if let Some(myentity) = passer.content.game_content.myentity
            && !passer.world.entities.contains_key(entity)
        {
            let client_pos = if let Some(Entity::Player(p_data)) =
                passer.world.entities.get(myentity)
            {
                p_data.pos
            } else {
                Position::default()
            };

            let npc = add_npc(
                passer.world,
                passer.systems,
                pos,
                client_pos.map,
                entity,
                num as usize,
            )?;
            create_npc_light(
                passer.world,
                passer.systems,
                &passer.content.game_content.game_lights,
                entity,
            );

            passer.content.game_content.npcs.borrow_mut().insert(npc);

            if let Some(Entity::Npc(n_data)) =
                passer.world.entities.get_mut(entity)
            {
                if let Some(npc_data) =
                    passer.systems.base.npc.get(num as usize)
                {
                    passer.systems.gfx.set_text(
                        &mut passer.systems.renderer,
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

            if passer.content.game_content.finalized {
                npc_finalized(
                    passer.world,
                    passer.systems,
                    entity,
                    passer.content.game_content.map.map_pos,
                    passer.content.game_content.game_lights,
                    false,
                )?;
                update_npc_camera(
                    passer.world,
                    passer.systems,
                    entity,
                    passer.socket,
                    &mut passer.content.game_content,
                )?;
            }
        }
    }

    Ok(())
}

pub fn handle_mapitems(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;
        let pos = data.read::<Position>()?;
        let item = data.read::<Item>()?;
        let _owner = data.read::<Option<GlobalKey>>()?;
        let _did_spawn = data.read::<bool>()?;

        if !passer.world.entities.contains_key(entity) {
            let sprite = if let Some(itemdata) =
                passer.systems.base.item.get(item.num as usize)
            {
                itemdata.sprite as usize
            } else {
                0
            };
            let mapitem = MapItem::create(
                passer.world,
                passer.systems,
                sprite,
                pos,
                entity,
            )?;

            passer
                .content
                .game_content
                .mapitems
                .borrow_mut()
                .insert(mapitem);

            if passer.content.game_content.finalized {
                MapItem::finalized(
                    passer.world,
                    passer.systems,
                    entity,
                    passer.content.game_content.game_lights,
                    false,
                )?;
                if let Some(Entity::MapItem(mi_data)) =
                    passer.world.entities.get(entity)
                {
                    update_mapitem_position(
                        passer.systems,
                        passer.content.game_content.game_lights,
                        mi_data.sprite_index,
                        &pos,
                        mi_data.pos_offset,
                        mi_data.light,
                    )?;
                }
            }
        }
    }

    Ok(())
}

pub fn handle_move(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;
        let pos = data.read::<Position>()?;
        let _warp = data.read::<bool>()?;
        let _switch = data.read::<bool>()?;
        let dir = data.read::<u8>()?;

        if let Some(myentity) = passer.content.game_content.myentity
            && passer.world.entities.contains_key(entity)
        {
            let player_pos = if let Some(Entity::Player(p_data)) =
                passer.world.entities.get(myentity)
            {
                p_data.pos
            } else {
                Position::default()
            };

            if is_map_connected(player_pos.map, pos.map) {
                if let Some(entity_data) = passer.world.entities.get_mut(entity)
                {
                    let movement_data = MovementData { end_pos: pos, dir };

                    match entity_data {
                        Entity::Player(p_data) => {
                            if p_data.movement_buffer.is_empty() {
                                p_data.movement_buffer.push_back(movement_data);
                            } else if let Some(data) =
                                p_data.movement_buffer.back()
                                && *data != movement_data
                            {
                                p_data.movement_buffer.push_back(movement_data);
                            }
                        }
                        Entity::Npc(n_data) => {
                            if n_data.movement_buffer.is_empty() {
                                n_data.movement_buffer.push_back(movement_data);
                            } else if let Some(data) =
                                n_data.movement_buffer.back()
                                && *data != movement_data
                            {
                                n_data.movement_buffer.push_back(movement_data);
                            }
                        }
                        _ => {}
                    }
                }
            } else if myentity != entity {
                let entity_kind = passer.world.get_kind(entity)?;

                match entity_kind {
                    EntityKind::Player => {
                        unload_player(
                            passer.world,
                            passer.systems,
                            &passer.content.game_content,
                            entity,
                        )?;
                        passer
                            .content
                            .game_content
                            .players
                            .borrow_mut()
                            .swap_remove(&entity);
                    }
                    EntityKind::Npc => {
                        unload_npc(
                            passer.world,
                            passer.systems,
                            &passer.content.game_content,
                            entity,
                        )?;
                        passer
                            .content
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

    Ok(())
}

pub fn handle_warp(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;
        let pos = data.read::<Position>()?;

        if !passer.world.entities.contains_key(entity) {
            continue;
        }

        let (old_pos, dir) =
            if let Some(entity_data) = passer.world.entities.get_mut(entity) {
                match entity_data {
                    Entity::Player(p_data) => {
                        let old_pos = p_data.pos;
                        p_data.movement_buffer.clear();
                        p_data.movement.is_moving = false;
                        p_data.pos = pos;
                        p_data.pos_offset = Vec2::ZERO;

                        (old_pos, p_data.dir)
                    }
                    Entity::Npc(n_data) => {
                        let old_pos = n_data.pos;
                        n_data.movement_buffer.clear();
                        n_data.movement.is_moving = false;
                        n_data.pos = pos;
                        n_data.pos_offset = Vec2::ZERO;

                        (old_pos, n_data.dir)
                    }
                    _ => {
                        continue;
                    }
                }
            } else {
                continue;
            };

        let world_entity_type = passer.world.get_kind(entity)?;

        if world_entity_type == EntityKind::Player {
            let frame = dir * PLAYER_SPRITE_FRAME_X as u8;
            set_player_frame(
                passer.world,
                passer.systems,
                entity,
                frame as usize,
            )?;
        } else if world_entity_type == EntityKind::Npc {
            let frame = dir * NPC_SPRITE_FRAME_X as u8;
            set_npc_frame(
                passer.world,
                passer.systems,
                entity,
                frame as usize,
            )?;
        }

        if world_entity_type == EntityKind::Player
            && let Some(myentity) = passer.content.game_content.myentity
        {
            let client_pos = if let Some(Entity::Player(p_data)) =
                passer.world.entities.get(myentity)
            {
                p_data.pos
            } else {
                Position::default()
            };

            if myentity == entity {
                //Removed Sends clear...
                //socket.client.sends.clear();

                if old_pos.map != pos.map {
                    passer.content.game_content.init_map(
                        passer.systems,
                        passer.map_renderer,
                        pos.map,
                        passer.buffer,
                        true,
                    )?;
                    finalize_entity(passer.world, passer.systems)?;
                    passer.content.game_content.refresh_map = true;
                }

                if passer.systems.map_fade.f_alpha > 0 {
                    passer.systems.map_fade.init_fade(
                        &mut passer.systems.gfx,
                        FadeType::Out,
                        0,
                        FadeData::None,
                    );
                }

                if let Some(target_entity) =
                    passer.content.game_content.target.entity
                    && let Some(entity_data) =
                        passer.world.entities.get_mut(target_entity)
                {
                    match entity_data {
                        Entity::Player(p_data) => {
                            passer
                                .systems
                                .gfx
                                .set_visible(&p_data.name_map.0, false);
                            passer
                                .systems
                                .gfx
                                .set_visible(&p_data.name_map.0, false);

                            passer.content.game_content.target.clear_target(
                                passer.socket,
                                passer.systems,
                                &mut p_data.hp_bar,
                            )?;
                        }
                        Entity::Npc(n_data) => {
                            passer
                                .systems
                                .gfx
                                .set_visible(&n_data.name_map.0, false);
                            passer
                                .systems
                                .gfx
                                .set_visible(&n_data.name_map.0, false);

                            passer.content.game_content.target.clear_target(
                                passer.socket,
                                passer.systems,
                                &mut n_data.hp_bar,
                            )?;
                        }
                        _ => {}
                    }
                }
            } else if !is_map_connected(client_pos.map, pos.map) {
                if let Some(target_entity) =
                    passer.content.game_content.target.entity
                    && target_entity == entity
                    && let Some(entity_data) =
                        passer.world.entities.get_mut(target_entity)
                {
                    match entity_data {
                        Entity::Player(p_data) => {
                            passer
                                .systems
                                .gfx
                                .set_visible(&p_data.name_map.0, false);
                            passer
                                .systems
                                .gfx
                                .set_visible(&p_data.name_map.0, false);

                            passer.content.game_content.target.clear_target(
                                passer.socket,
                                passer.systems,
                                &mut p_data.hp_bar,
                            )?;
                        }
                        Entity::Npc(n_data) => {
                            passer
                                .systems
                                .gfx
                                .set_visible(&n_data.name_map.0, false);
                            passer
                                .systems
                                .gfx
                                .set_visible(&n_data.name_map.0, false);

                            passer.content.game_content.target.clear_target(
                                passer.socket,
                                passer.systems,
                                &mut n_data.hp_bar,
                            )?;
                        }
                        _ => {}
                    }
                }

                unload_player(
                    passer.world,
                    passer.systems,
                    &passer.content.game_content,
                    entity,
                )?;
                passer
                    .content
                    .game_content
                    .players
                    .borrow_mut()
                    .swap_remove(&entity);
                continue;
            }

            update_player_camera(
                passer.world,
                passer.systems,
                passer.socket,
                entity,
                &mut passer.content.game_content,
            )?;
        }
    }

    Ok(())
}

pub fn handle_dir(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;
        let dir = data.read::<u8>()?;

        if passer.world.entities.contains_key(entity) {
            let is_moving = if let Some(entity_data) =
                passer.world.entities.get_mut(entity)
            {
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
                let entity_kind = passer.world.get_kind(entity)?;

                if entity_kind == EntityKind::Player {
                    let frame = dir * PLAYER_SPRITE_FRAME_X as u8;
                    set_player_frame(
                        passer.world,
                        passer.systems,
                        entity,
                        frame as usize,
                    )?;
                } else if entity_kind == EntityKind::Npc {
                    let frame = dir * NPC_SPRITE_FRAME_X as u8;
                    set_npc_frame(
                        passer.world,
                        passer.systems,
                        entity,
                        frame as usize,
                    )?;
                };
            }
        }
    }

    Ok(())
}

pub fn handle_vitals(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;
        let mut vitals = [0; VITALS_MAX];
        vitals.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);
        let mut vitalmax = [0; VITALS_MAX];
        vitalmax.copy_from_slice(&data.read::<[i32; VITALS_MAX]>()?);

        if passer.world.entities.contains_key(entity) {
            let hpbar = if let Some(entity_data) =
                passer.world.entities.get_mut(entity)
            {
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

            let mut size = passer.systems.gfx.get_size(&hpbar.bar_index);
            size.x = get_percent(vitals[0], vitalmax[0], 18) as f32;
            passer.systems.gfx.set_size(&hpbar.bar_index, size);

            let entity_kind = passer.world.get_kind(entity)?;

            if entity_kind == EntityKind::Player
                && let Some(myentity) = passer.content.game_content.myentity
                && entity == myentity
            {
                if let Some(Entity::Player(p_data)) =
                    passer.world.entities.get_mut(entity)
                {
                    p_data.hp_bar.visible = vitals[0] != vitalmax[0];
                }

                passer
                    .systems
                    .gfx
                    .set_visible(&hpbar.bar_index, vitals[0] != vitalmax[0]);
                passer
                    .systems
                    .gfx
                    .set_visible(&hpbar.bg_index, vitals[0] != vitalmax[0]);

                passer
                    .content
                    .game_content
                    .interface
                    .vitalbar
                    .update_bar_size(passer.systems, 0, vitals[0], vitalmax[0]);
                passer
                    .content
                    .game_content
                    .interface
                    .vitalbar
                    .update_bar_size(passer.systems, 1, vitals[2], vitalmax[2]);
            }
        }
    }

    Ok(())
}

pub fn handle_attack(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;

        if passer.world.entities.contains_key(entity) {
            let entity_kind = passer.world.get_kind(entity)?;

            match entity_kind {
                EntityKind::Player => {
                    if let Some(myentity) = passer.content.game_content.myentity
                        && myentity != entity
                    {
                        init_player_attack(
                            passer.world,
                            passer.systems,
                            entity,
                            passer.seconds,
                        )?
                    }
                }
                EntityKind::Npc => {
                    init_npc_attack(
                        passer.world,
                        passer.systems,
                        entity,
                        passer.seconds,
                    )?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

pub fn handle_death(
    data: &mut MByteBuffer,
    _passer: &mut PacketPasser,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let _entity = data.read::<GlobalKey>()?;
        let _deathtype = data.read::<DeathType>()?;
    }

    Ok(())
}

pub fn handle_entityunload(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let entity = data.read::<GlobalKey>()?;

        if passer.world.entities.contains_key(entity) {
            if let Some(target_entity) =
                passer.content.game_content.target.entity
                && target_entity == entity
                && let Some(entity_data) =
                    passer.world.entities.get_mut(target_entity)
            {
                match entity_data {
                    Entity::Player(p_data) => {
                        passer.content.game_content.target.clear_target(
                            passer.socket,
                            passer.systems,
                            &mut p_data.hp_bar,
                        )?;
                    }
                    Entity::Npc(n_data) => {
                        passer.content.game_content.target.clear_target(
                            passer.socket,
                            passer.systems,
                            &mut n_data.hp_bar,
                        )?;
                    }
                    _ => {}
                }
            }

            let world_entity_type = passer.world.get_kind(entity)?;
            match world_entity_type {
                EntityKind::Player => {
                    unload_player(
                        passer.world,
                        passer.systems,
                        &passer.content.game_content,
                        entity,
                    )?;
                    passer
                        .content
                        .game_content
                        .players
                        .borrow_mut()
                        .swap_remove(&entity);
                }
                EntityKind::Npc => {
                    unload_npc(
                        passer.world,
                        passer.systems,
                        &passer.content.game_content,
                        entity,
                    )?;
                    passer
                        .content
                        .game_content
                        .npcs
                        .borrow_mut()
                        .swap_remove(&entity);
                }
                EntityKind::MapItem => {
                    unload_mapitems(
                        passer.world,
                        passer.systems,
                        &passer.content.game_content,
                        entity,
                    )?;
                    passer
                        .content
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
