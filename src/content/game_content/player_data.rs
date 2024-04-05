use hecs::World;

use crate::{
    Entity, Equipment, EquipmentType, IsUsingType, Item, Result, SystemHolder,
};

pub struct PlayerData {
    pub inventory: Vec<Item>,
    pub storage: Vec<Item>,
    pub equipment: Vec<Item>,
    pub player_money: u64,
    pub levelexp: u64,
    pub is_using_type: IsUsingType,
}

impl PlayerData {
    pub fn new() -> Self {
        PlayerData {
            inventory: Vec::new(),
            storage: Vec::new(),
            equipment: Vec::new(),
            player_money: 0,
            levelexp: 0,
            is_using_type: IsUsingType::None,
        }
    }

    pub fn unload(&mut self) {
        self.inventory.clear();
        self.storage.clear();
        self.equipment.clear();
    }
}

pub fn player_get_weapon_damage(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
) -> Result<(i16, i16)> {
    let mut query = world.query_one::<&mut Equipment>(entity.0)?;

    Ok(if let Some(player_equipment) = query.get() {
        let mut dmg = (0, 0);

        if player_equipment.items[EquipmentType::Weapon as usize].val > 0 {
            if let Some(item) = systems.base.item.get(
                player_equipment.items[EquipmentType::Weapon as usize].num
                    as usize,
            ) {
                dmg = (item.data[0], item.data[1]);
            }
        }

        dmg
    } else {
        (0, 0)
    })
}

pub fn player_get_armor_defense(
    world: &mut World,
    systems: &mut SystemHolder,
    entity: &Entity,
) -> Result<(i16, i16)> {
    let mut query = world.query_one::<&mut Equipment>(entity.0)?;

    Ok(if let Some(player_equipment) = query.get() {
        let mut defense = (0i16, 0i16);

        for i in
            EquipmentType::Helmet as usize..=EquipmentType::Accessory as usize
        {
            if let Some(item) = systems
                .base
                .item
                .get(player_equipment.items[i].num as usize)
            {
                defense.0 = defense.0.saturating_add(item.data[0]);
                defense.1 = defense.1.saturating_add(item.data[1]);
            }
        }

        defense
    } else {
        (0, 0)
    })
}
