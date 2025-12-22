use mmap_bytey::MByteBuffer;

use crate::{
    Alert, DeathType, Entity, Equipment, GlobalKey, IsUsingType, Item,
    MAX_EQPT, Position, Result, UserAccess, VITALS_MAX, World,
    content::{
        Content, ProfileLabel, Window, add_player, close_interface,
        create_player_light, player_get_armor_defense, player_get_next_lvl_exp,
        player_get_weapon_damage,
    },
    systems::{BufferTask, Poller, SystemHolder, mapper::PacketPasser},
};

pub fn handle_playerdata(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    if let Some(entity) = passer.content.game_content.myentity {
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

        if !passer.world.entities.contains_key(entity)
            || !passer.content.game_content.in_game
        {
            let player = add_player(
                passer.world,
                passer.systems,
                pos,
                pos.map,
                entity,
                sprite as usize,
            )?;
            // Create Lights
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
            passer.content.game_content.in_game = true;
        }

        passer.content.game_content.player_data.equipment[..]
            .copy_from_slice(&equipment.items);

        if let Some(Entity::Player(p_data)) =
            passer.world.entities.get_mut(entity)
        {
            passer.systems.gfx.set_text(
                &mut passer.systems.renderer,
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

pub fn handle_playerinv(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let items = data.read::<Vec<Item>>()?;

    passer.content.game_content.player_data.inventory[..]
        .copy_from_slice(&items);

    if passer.content.game_content.finalized {
        for (index, item) in items.iter().enumerate() {
            passer
                .content
                .game_content
                .interface
                .inventory
                .update_inv_slot(passer.systems, index, item);
        }
    }

    Ok(())
}

pub fn handle_playerinvslot(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let index = data.read::<usize>()?;
    let item = data.read::<Item>()?;

    passer
        .content
        .game_content
        .interface
        .inventory
        .update_inv_slot(passer.systems, index, &item);

    Ok(())
}

pub fn handle_playerstorage(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let start = data.read::<usize>()?;
    let end = data.read::<usize>()?;
    let items = data.read::<Vec<Item>>()?;

    passer.content.game_content.player_data.storage[start..end]
        .copy_from_slice(&items);
    if passer.content.game_content.finalized {
        for (index, item) in passer.content.game_content.player_data.storage
            [start..end]
            .iter()
            .enumerate()
        {
            passer
                .content
                .game_content
                .interface
                .storage
                .update_storage_slot(passer.systems, index + start, item);
        }
    }

    Ok(())
}

pub fn handle_playerstorageslot(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let index = data.read::<usize>()?;
    let item = data.read::<Item>()?;

    passer
        .content
        .game_content
        .interface
        .storage
        .update_storage_slot(passer.systems, index, &item);

    Ok(())
}

pub fn handle_playerequipment(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let entity = data.read::<GlobalKey>()?;
    let equipment = data.read::<Equipment>()?;

    let (b_damage, b_defense) = if let Some(Entity::Player(p_data)) =
        passer.world.entities.get_mut(entity)
    {
        p_data.equipment.clone_from(&equipment);

        (p_data.physical.damage, p_data.physical.defense)
    } else {
        return Ok(());
    };

    if let Some(myentity) = passer.content.game_content.myentity
        && myentity == entity
    {
        for i in 0..MAX_EQPT {
            if passer.content.game_content.player_data.equipment[i]
                != equipment.items[i]
            {
                passer
                    .content
                    .game_content
                    .interface
                    .profile
                    .update_equipment_slot(
                        passer.systems,
                        i,
                        &equipment.items[i],
                    );
                passer.content.game_content.player_data.equipment[i] =
                    equipment.items[i];
            }
        }

        let damage = b_damage.saturating_add(
            player_get_weapon_damage(passer.world, passer.systems, myentity)?.0
                as u32,
        );
        passer
            .content
            .game_content
            .interface
            .profile
            .set_profile_label_value(
                passer.systems,
                ProfileLabel::Damage,
                damage as u64,
            );
        let defense = b_defense.saturating_add(
            player_get_armor_defense(passer.world, passer.systems, myentity)?.0
                as u32,
        );
        passer
            .content
            .game_content
            .interface
            .profile
            .set_profile_label_value(
                passer.systems,
                ProfileLabel::Defense,
                defense as u64,
            );
    }

    Ok(())
}

pub fn handle_playerlevel(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let level = data.read::<i32>()?;
    let levelexp = data.read::<u64>()?;

    passer.content.game_content.player_data.levelexp = levelexp;

    if let Some(myentity) = passer.content.game_content.myentity
        && passer.world.entities.contains_key(myentity)
    {
        if let Some(Entity::Player(p_data)) =
            passer.world.entities.get_mut(myentity)
        {
            p_data.level = level;
        }

        let nextexp = player_get_next_lvl_exp(passer.world, myentity)?;

        if passer.content.game_content.finalized {
            passer
                .content
                .game_content
                .interface
                .vitalbar
                .update_bar_size(
                    passer.systems,
                    2,
                    levelexp as i32,
                    nextexp as i32,
                );

            passer
                .content
                .game_content
                .interface
                .profile
                .set_profile_label_value(
                    passer.systems,
                    ProfileLabel::Level,
                    level as u64,
                );
        }
    }

    Ok(())
}

pub fn handle_playermoney(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let vals = data.read::<u64>()?;

    passer.content.game_content.player_data.player_money = vals;
    passer
        .content
        .game_content
        .interface
        .profile
        .set_profile_label_value(passer.systems, ProfileLabel::Money, vals);

    Ok(())
}

pub fn handle_playerpk(
    _data: &mut MByteBuffer,
    _passer: &mut PacketPasser,
) -> Result<()> {
    Ok(())
}

pub fn handle_clearisusingtype(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let _ = data.read::<u16>()?;

    match passer.content.game_content.player_data.is_using_type {
        IsUsingType::Bank => close_interface(
            &mut passer.content.game_content.interface,
            passer.systems,
            Window::Storage,
        ),
        IsUsingType::Store(_) => close_interface(
            &mut passer.content.game_content.interface,
            passer.systems,
            Window::Shop,
        ),
        IsUsingType::Trading(_) => {
            close_interface(
                &mut passer.content.game_content.interface,
                passer.systems,
                Window::Trade,
            );
            passer
                .content
                .game_content
                .interface
                .trade
                .clear_trade_items(passer.systems);
        }
        _ => {}
    }

    passer.content.game_content.player_data.is_using_type = IsUsingType::None;

    Ok(())
}
