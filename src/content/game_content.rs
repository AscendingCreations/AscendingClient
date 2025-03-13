use std::{cell::RefCell, rc::Rc};

use graphics::*;

use indexmap::IndexSet;

pub mod content_input;
pub mod interface;

pub use content_input::*;
pub use interface::*;

use crate::{
    Direction, Entity, EntityKind, GlobalKey, IsUsingType, MapPosition,
    MovementType, Position, Result, SystemHolder, TILE_SIZE, content::*,
    data_types::*, database::*, logic::*, send_attack, send_pickup, systems::*,
};

pub mod floating_text;
pub mod map;
pub mod npc;
pub mod player;
pub mod player_data;
pub mod target;

pub use floating_text::*;
pub use map::*;
pub use npc::*;
pub use player::*;
pub use player_data::*;
pub use target::*;

const KEY_ATTACK: usize = 0;
const KEY_MOVEUP: usize = 1;
const KEY_MOVELEFT: usize = 2;
const KEY_MOVEDOWN: usize = 3;
const KEY_MOVERIGHT: usize = 4;
const KEY_PICKUP: usize = 5;
const MAX_KEY: usize = 6;

#[derive(Clone, Debug)]
pub struct Camera(pub Vec2);

impl Camera {
    pub fn new(tile_pos: Vec2) -> Self {
        Self(tile_pos * TILE_SIZE as f32)
    }
}

pub struct GameContent {
    pub players: Rc<RefCell<IndexSet<GlobalKey, ahash::RandomState>>>,
    pub npcs: Rc<RefCell<IndexSet<GlobalKey, ahash::RandomState>>>,
    pub mapitems: Rc<RefCell<IndexSet<GlobalKey, ahash::RandomState>>>,
    pub game_lights: GfxType,
    pub map: MapContent,
    camera: Camera,
    pub interface: Interface,
    pub keyinput: [bool; MAX_KEY],
    pub myentity: Option<GlobalKey>,
    pub in_game: bool,
    pub player_data: PlayerData,
    pub finalized: bool,
    pub target: Target,
    pub pick_up_timer: f32,
    pub current_music: String,
    pub float_text: FloatingText,
    pub refresh_map: bool,
    pub can_move: bool,
    pub reconnect_count: usize,
}

impl GameContent {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let mut lights = Lights::new(&mut systems.renderer, 0, ORDER_LIGHT);
        lights.world_color = Vec4::new(0.0, 0.0, 0.0, 0.8);
        lights.enable_lights = true;

        let game_lights =
            systems.gfx.add_light(lights, 1, "Game Lights", false);

        GameContent {
            players: Rc::new(RefCell::new(IndexSet::default())),
            npcs: Rc::new(RefCell::new(IndexSet::default())),
            mapitems: Rc::new(RefCell::new(IndexSet::default())),
            game_lights,
            map: MapContent::new(),
            camera: Camera::new(Vec2::new(0.0, 0.0)),
            interface: Interface::new(systems),
            keyinput: [false; MAX_KEY],
            finalized: false,
            myentity: None,
            in_game: false,
            player_data: PlayerData::new(),
            target: Target::new(systems),
            pick_up_timer: 0.0,
            current_music: String::new(),
            float_text: FloatingText::new(),
            refresh_map: false,
            can_move: true,
            reconnect_count: 0,
        }
    }

    pub fn show(&mut self, systems: &mut SystemHolder) {
        self.map.recreate();
        self.interface.recreate(systems);
        self.keyinput.iter_mut().for_each(|key| {
            *key = false;
        });
        self.target.recreate(systems);
        self.float_text.recreate();
        self.finalized = false;
    }

    pub fn clear_data(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
    ) -> Result<()> {
        for entity in self.players.borrow().iter() {
            unload_player(world, systems, self, *entity)?;
        }
        for entity in self.npcs.borrow().iter() {
            unload_npc(world, systems, self, *entity)?;
        }
        for entity in self.mapitems.borrow().iter() {
            unload_mapitems(world, systems, self, *entity)?;
        }
        systems.gfx.set_visible(&self.game_lights, false);
        self.players.borrow_mut().clear();
        self.npcs.borrow_mut().clear();
        self.mapitems.borrow_mut().clear();
        self.finalized = false;
        self.myentity = None;
        self.interface.unload(systems);
        self.target.unload(systems);
        self.map.unload(systems);
        self.player_data.unload();
        self.float_text.unload(systems);
        systems.caret.index = None;
        Ok(())
    }

    pub fn hide(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
    ) -> Result<()> {
        self.clear_data(world, systems)
    }

    pub fn finalize_entity(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Poller,
    ) -> Result<()> {
        for entity in self.players.borrow().iter() {
            player_finalized(world, systems, *entity)?;
        }
        for entity in self.npcs.borrow().iter() {
            npc_finalized(world, systems, *entity)?;
        }
        for entity in self.mapitems.borrow().iter() {
            MapItem::finalized(world, systems, *entity)?;
        }
        update_camera(world, self, systems, socket)?;

        Ok(())
    }

    pub fn init_finalized_data(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Poller,
    ) -> Result<()> {
        self.finalize_entity(world, systems, socket)?;

        self.player_data.inventory.iter().enumerate().for_each(
            |(index, item)| {
                self.interface
                    .inventory
                    .update_inv_slot(systems, index, item);
            },
        );
        self.player_data.storage.iter().enumerate().for_each(
            |(index, item)| {
                self.interface
                    .storage
                    .update_storage_slot(systems, index, item);
            },
        );

        if let Some(myindex) = self.myentity {
            let weapon_damage =
                player_get_weapon_damage(world, systems, myindex)?.0 as u32;
            let armor_defense =
                player_get_armor_defense(world, systems, myindex)?.0 as u32;
            let nextexp = player_get_next_lvl_exp(world, myindex)?;

            if let Some(Entity::Player(p_data)) = world.entities.get(myindex) {
                self.interface.profile.set_profile_label_value(
                    systems,
                    ProfileLabel::Level,
                    p_data.level as u64,
                );
                self.interface.profile.set_profile_label_value(
                    systems,
                    ProfileLabel::Money,
                    self.player_data.player_money,
                );
                let damage =
                    p_data.physical.damage.saturating_add(weapon_damage);
                self.interface.profile.set_profile_label_value(
                    systems,
                    ProfileLabel::Damage,
                    damage as u64,
                );
                let defense =
                    p_data.physical.defense.saturating_add(armor_defense);
                self.interface.profile.set_profile_label_value(
                    systems,
                    ProfileLabel::Defense,
                    defense as u64,
                );

                self.interface.vitalbar.update_bar_size(
                    systems,
                    0,
                    p_data.vitals.vital[0],
                    p_data.vitals.vitalmax[0],
                );
                self.interface.vitalbar.update_bar_size(
                    systems,
                    1,
                    p_data.vitals.vital[2],
                    p_data.vitals.vitalmax[2],
                );

                self.interface.vitalbar.update_bar_size(
                    systems,
                    2,
                    self.player_data.levelexp as i32,
                    nextexp as i32,
                );

                let mut size = systems.gfx.get_size(&p_data.hp_bar.bar_index);
                size.x = get_percent(
                    p_data.vitals.vital[0],
                    p_data.vitals.vitalmax[0],
                    18,
                ) as f32;
                systems.gfx.set_size(&p_data.hp_bar.bar_index, size);
                systems.gfx.set_visible(
                    &p_data.hp_bar.bar_index,
                    p_data.hp_bar.visible,
                );
                systems.gfx.set_visible(
                    &p_data.hp_bar.bg_index,
                    p_data.hp_bar.visible,
                );
            }
        }

        for i in 0..MAX_EQPT {
            self.interface.profile.update_equipment_slot(
                systems,
                i,
                &self.player_data.equipment[i],
            );
        }

        systems.gfx.set_visible(&self.game_lights, true);

        if let Some(music) = &get_map_music(systems, self.map.mapindex[0]) {
            if self.current_music != *music {
                self.current_music.clone_from(music);
                systems.audio.set_music(format!("./audio/{}", music))?;
            }
        }

        self.finalized = true;
        Ok(())
    }

    pub fn init_map(
        &mut self,
        systems: &mut SystemHolder,
        map: MapPosition,
        buffer: &mut BufferTask,
    ) -> Result<()> {
        self.map.map_pos = map;

        for i in 0..9 {
            let (mx, my) = get_map_loc(map.x, map.y, i);

            if let Some(mappos) = get_map_id(systems, self.map.mapindex[i]) {
                if map.checkdistance(mappos) > 1 {
                    set_map_visible(systems, self.map.mapindex[i], false);
                }
            }

            let key = get_map_key(systems, mx, my, map.group, buffer)?;
            self.map.mapindex[i] = key;
            set_map_pos(systems, key, get_mapindex_base_pos(i));
            set_map_visible(systems, key, true);
        }

        Ok(())
    }

    pub fn move_map(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Poller,
        dir: Direction,
        buffer: &mut BufferTask,
    ) -> Result<()> {
        match dir {
            Direction::Down => self.map.map_pos.y -= 1,
            Direction::Left => self.map.map_pos.x -= 1,
            Direction::Right => self.map.map_pos.x += 1,
            Direction::Up => self.map.map_pos.y += 1,
        }

        self.init_map(systems, self.map.map_pos, buffer)?;

        update_camera(world, self, systems, socket)
    }

    pub fn handle_key_input(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Poller,
        seconds: f32,
    ) -> Result<()> {
        if self.interface.selected_textbox != SelectedTextbox::None {
            return Ok(());
        }

        for i in 0..MAX_KEY {
            if self.keyinput[i] {
                match i {
                    KEY_ATTACK => {
                        self.player_attack(world, systems, socket, seconds)?
                    }
                    KEY_MOVEDOWN => self.move_player(
                        world,
                        systems,
                        socket,
                        &Direction::Down,
                    )?,
                    KEY_MOVELEFT => self.move_player(
                        world,
                        systems,
                        socket,
                        &Direction::Left,
                    )?,
                    KEY_MOVEUP => self.move_player(
                        world,
                        systems,
                        socket,
                        &Direction::Up,
                    )?,
                    KEY_MOVERIGHT => self.move_player(
                        world,
                        systems,
                        socket,
                        &Direction::Right,
                    )?,
                    KEY_PICKUP => {
                        if self.pick_up_timer < seconds {
                            send_pickup(socket)?;
                            self.pick_up_timer = seconds + 1.0;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn move_player(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Poller,
        dir: &Direction,
    ) -> Result<()> {
        if let Some(myentity) = self.myentity {
            match self.player_data.is_using_type {
                IsUsingType::Bank => send_closestorage(socket)?,
                IsUsingType::Store(_) => send_closeshop(socket)?,
                _ => {}
            }

            move_player(
                world,
                systems,
                socket,
                myentity,
                self,
                MovementType::Manual(enum_to_dir(*dir), None),
            )?;
        }
        Ok(())
    }

    pub fn player_attack(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Poller,
        seconds: f32,
    ) -> Result<()> {
        if let Some(myentity) = self.myentity {
            let (pos, dir) = if let Some(Entity::Player(p_data)) =
                world.entities.get(myentity)
            {
                if p_data.attacking.0 || p_data.movement.is_moving {
                    return Ok(());
                }

                (p_data.pos, p_data.dir)
            } else {
                return Ok(());
            };

            let target_pos = match dir_to_enum(dir) {
                Direction::Down => {
                    let mut next_pos = pos;
                    next_pos.y -= 1;
                    if next_pos.y < 0 {
                        next_pos.y = 31;
                        next_pos.map.y -= 1;
                    }
                    next_pos
                }
                Direction::Left => {
                    let mut next_pos = pos;
                    next_pos.x -= 1;
                    if next_pos.x < 0 {
                        next_pos.x = 31;
                        next_pos.map.x -= 1;
                    }
                    next_pos
                }
                Direction::Right => {
                    let mut next_pos = pos;
                    next_pos.x += 1;
                    if next_pos.x >= 32 {
                        next_pos.x = 0;
                        next_pos.map.x += 1;
                    }
                    next_pos
                }
                Direction::Up => {
                    let mut next_pos = pos;
                    next_pos.y += 1;
                    if next_pos.y >= 32 {
                        next_pos.y = 0;
                        next_pos.map.y += 1;
                    }
                    next_pos
                }
            };

            let target_entity =
                world.entities.iter().find_map(|(key, entity_data)| {
                    match entity_data {
                        Entity::Player(p_data) => {
                            if p_data.pos == target_pos {
                                return Some(key);
                            }
                        }
                        Entity::Npc(n_data) => {
                            if n_data.pos == target_pos {
                                return Some(key);
                            }
                        }
                        _ => {}
                    }
                    None
                });

            if let Some(got_target) = target_entity {
                let proceed_target = if let Some(t_entity) = self.target.entity
                {
                    if t_entity != got_target {
                        if let Some(Entity::Player(p_data)) =
                            world.entities.get_mut(t_entity)
                        {
                            self.target.clear_target(
                                socket,
                                systems,
                                &mut p_data.hp_bar,
                            )?;
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    true
                };

                if proceed_target {
                    self.target.set_target(socket, systems, got_target)?;
                    let entity_kind = world.get_kind(got_target)?;
                    match entity_kind {
                        EntityKind::Player => {
                            update_player_camera(
                                world, systems, socket, got_target, self,
                            )?;
                        }
                        EntityKind::Npc => {
                            update_npc_camera(
                                world, systems, got_target, socket, self,
                            )?;
                        }
                        _ => {}
                    }
                }
            }

            send_attack(socket, dir, target_entity)?;
            init_player_attack(world, systems, myentity, seconds)?;
        }

        Ok(())
    }
}

pub fn update_player(
    world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Poller,
    content: &mut GameContent,
    buffer: &mut BufferTask,
    seconds: f32,
) -> Result<()> {
    let players = content.players.clone();
    for entity in players.borrow().iter() {
        if let Some(myentity) = content.myentity {
            if entity != &myentity {
                move_player(
                    world,
                    systems,
                    socket,
                    *entity,
                    content,
                    MovementType::MovementBuffer,
                )?;
            }
        }

        process_player_movement(
            world, systems, socket, *entity, content, buffer,
        )?;
        process_player_attack(world, systems, *entity, seconds)?
    }
    Ok(())
}

pub fn update_npc(
    world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Poller,
    content: &mut GameContent,
    seconds: f32,
) -> Result<()> {
    let npcs = content.npcs.clone();
    for entity in npcs.borrow().iter() {
        move_npc(world, systems, *entity, MovementType::MovementBuffer)?;
        process_npc_movement(world, systems, *entity, socket, content)?;
        process_npc_attack(world, systems, *entity, seconds)?;
    }
    Ok(())
}

pub fn finalize_entity(
    world: &mut World,
    systems: &mut SystemHolder,
) -> Result<()> {
    for (_, entity) in world.entities.iter_mut() {
        match entity {
            Entity::Player(p_data) => {
                if !p_data.finalized {
                    player_finalized_data(
                        systems,
                        p_data.sprite_index.0,
                        p_data.name_map.0,
                        &p_data.hp_bar,
                    );
                    p_data.finalized = true;
                }
            }
            Entity::Npc(n_data) => {
                if !n_data.finalized {
                    npc_finalized_data(
                        systems,
                        n_data.sprite_index.0,
                        n_data.name_map.0,
                        &n_data.hp_bar,
                    );
                    n_data.finalized = true;
                }
            }
            Entity::MapItem(i_data) => {
                if !i_data.finalized {
                    MapItem::finalized_data(systems, i_data.sprite_index);
                    i_data.finalized = true;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn update_camera(
    world: &mut World,
    content: &mut GameContent,
    systems: &mut SystemHolder,
    socket: &mut Poller,
) -> Result<()> {
    let player_pos = if let Some(entity) = content.myentity {
        if let Some(Entity::Player(p_data)) = world.entities.get_mut(entity) {
            (Vec2::new(p_data.pos.x as f32, p_data.pos.y as f32)
                * TILE_SIZE as f32)
                + p_data.pos_offset
        } else {
            return Ok(());
        }
    } else {
        Vec2::new(0.0, 0.0)
    };

    let adjust_pos = get_screen_center(&systems.size) - player_pos;
    if content.camera.0 == adjust_pos {
        return Ok(());
    }

    content.camera.0 = adjust_pos;

    content.map.move_pos(systems, content.camera.0);

    for (key, entity) in world.entities.iter_mut() {
        let is_target = content.target.entity == Some(key);
        let is_my_entity = content.myentity == Some(key);

        match entity {
            Entity::Player(p_data) => {
                update_player_position(
                    systems,
                    content,
                    p_data.sprite_index.0,
                    &p_data.pos,
                    p_data.pos_offset,
                    &p_data.hp_bar,
                    &p_data.name_map,
                    p_data.light,
                )?;
                if is_target {
                    if !p_data.hp_bar.visible {
                        p_data.hp_bar.visible = true;
                        systems.gfx.set_visible(&p_data.hp_bar.bar_index, true);
                        systems.gfx.set_visible(&p_data.hp_bar.bg_index, true);
                    }
                    let spritepos = systems.gfx.get_pos(&p_data.sprite_index.0);
                    content.target.set_target_pos(
                        socket,
                        systems,
                        Vec2::new(spritepos.x, spritepos.y),
                        &mut p_data.hp_bar,
                    )?;
                } else if p_data.hp_bar.visible && !is_my_entity {
                    p_data.hp_bar.visible = false;
                    systems.gfx.set_visible(&p_data.hp_bar.bar_index, false);
                    systems.gfx.set_visible(&p_data.hp_bar.bg_index, false);
                }
            }
            Entity::Npc(n_data) => {
                update_npc_position(
                    systems,
                    content,
                    n_data.sprite_index.0,
                    &n_data.pos,
                    n_data.pos_offset,
                    &n_data.hp_bar,
                    &n_data.name_map,
                    n_data.light,
                )?;
                if is_target {
                    if !n_data.hp_bar.visible {
                        n_data.hp_bar.visible = true;
                        systems.gfx.set_visible(&n_data.hp_bar.bar_index, true);
                        systems.gfx.set_visible(&n_data.hp_bar.bg_index, true);
                    }
                    let spritepos = systems.gfx.get_pos(&n_data.sprite_index.0);
                    content.target.set_target_pos(
                        socket,
                        systems,
                        Vec2::new(spritepos.x, spritepos.y),
                        &mut n_data.hp_bar,
                    )?;
                } else if n_data.hp_bar.visible {
                    n_data.hp_bar.visible = false;
                    systems.gfx.set_visible(&n_data.hp_bar.bar_index, false);
                    systems.gfx.set_visible(&n_data.hp_bar.bg_index, false);
                }
            }
            Entity::MapItem(i_data) => {
                update_mapitem_position(
                    systems,
                    content,
                    i_data.sprite_index,
                    &i_data.pos,
                    i_data.pos_offset,
                    i_data.light,
                );
            }
            _ => {}
        }
    }
    Ok(())
}
