use graphics::*;

use indexmap::IndexSet;

pub mod content_input;
pub mod interface;

pub use content_input::*;
pub use interface::*;

use crate::{
    content::*, data_types::*, database::*, logic::*, send_attack, send_pickup,
    systems::*, Direction, Result, Socket, SystemHolder,
};
use hecs::World;

mod camera;
pub mod floating_text;
pub mod map;
pub mod npc;
pub mod player;
pub mod player_data;
pub mod target;

use camera::*;
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

pub struct GameContent {
    pub players: IndexSet<Entity>,
    pub npcs: IndexSet<Entity>,
    pub mapitems: IndexSet<Entity>,
    pub map: MapContent,
    camera: Camera,
    pub interface: Interface,
    keyinput: [bool; MAX_KEY],
    pub myentity: Option<Entity>,
    pub in_game: bool,
    pub player_data: PlayerData,
    pub finalized: bool,
    pub target: Target,
    pub pick_up_timer: f32,
    pub current_music: String,
    pub float_text: FloatingText,
}

impl GameContent {
    pub fn new(systems: &mut SystemHolder) -> Self {
        GameContent {
            players: IndexSet::default(),
            npcs: IndexSet::default(),
            mapitems: IndexSet::default(),
            map: MapContent::new(systems),
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
        }
    }

    pub fn show(&mut self, systems: &mut SystemHolder) {
        self.map.recreate(systems);
        self.interface.recreate(systems);
        self.keyinput.iter_mut().for_each(|key| {
            *key = false;
        });
        self.target.recreate(systems);
        self.float_text.recreate();
        self.finalized = false;
    }

    pub fn hide(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
    ) -> Result<()> {
        for entity in self.players.iter() {
            unload_player(world, systems, entity)?;
        }
        for entity in self.npcs.iter() {
            unload_npc(world, systems, entity)?;
        }
        for entity in self.mapitems.iter() {
            unload_mapitems(world, systems, entity)?;
        }
        self.players.clear();
        self.npcs.clear();
        self.mapitems.clear();
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

    pub fn init_map(
        &mut self,
        systems: &mut SystemHolder,
        map: MapPosition,
    ) -> Result<()> {
        self.map.map_pos = map;

        self.map.map_attribute.clear();
        self.map.dir_block.clear();
        self.map.music.clear();

        for i in 0..9 {
            let (mx, my) = get_map_loc(map.x, map.y, i);
            let mapdata = load_file(mx, my, map.group as u64)?;
            load_map_data(systems, &mapdata, self.map.index[i].0);

            self.map.map_attribute.push((
                MapAttributes {
                    attribute: mapdata.attribute.clone(),
                },
                i,
            ));

            self.map.music.push((mapdata.music.clone(), i));

            self.map.dir_block.push((
                MapDirBlock {
                    dir: mapdata.dir_block.clone(),
                },
                i,
            ));
        }
        Ok(())
    }

    pub fn finalize_entity(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Socket,
    ) -> Result<()> {
        for entity in self.players.iter() {
            player_finalized(world, systems, entity)?;
        }
        for entity in self.npcs.iter() {
            npc_finalized(world, systems, entity)?;
        }
        for entity in self.mapitems.iter() {
            MapItem::finalized(world, systems, entity)?;
        }
        update_camera(world, self, systems, socket)?;

        Ok(())
    }

    pub fn init_finalized_data(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Socket,
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
            self.interface.profile.set_profile_label_value(
                systems,
                ProfileLabel::Level,
                world.get_or_err::<Level>(&myindex)?.0 as u64,
            );
            self.interface.profile.set_profile_label_value(
                systems,
                ProfileLabel::Money,
                self.player_data.player_money,
            );
            let damage = world
                .get_or_err::<Physical>(&myindex)?
                .damage
                .saturating_add(
                    player_get_weapon_damage(world, systems, &myindex)?.0
                        as u32,
                );
            self.interface.profile.set_profile_label_value(
                systems,
                ProfileLabel::Damage,
                damage as u64,
            );
            let defense = world
                .get_or_err::<Physical>(&myindex)?
                .defense
                .saturating_add(
                    player_get_armor_defense(world, systems, &myindex)?.0
                        as u32,
                );
            self.interface.profile.set_profile_label_value(
                systems,
                ProfileLabel::Defense,
                defense as u64,
            );

            let vitals = world.get_or_err::<Vitals>(&myindex)?;
            self.interface.vitalbar.update_bar_size(
                systems,
                0,
                vitals.vital[0],
                vitals.vitalmax[0],
            );
            self.interface.vitalbar.update_bar_size(
                systems,
                1,
                vitals.vital[2],
                vitals.vitalmax[2],
            );

            let nextexp = player_get_next_lvl_exp(world, &myindex)?;
            self.interface.vitalbar.update_bar_size(
                systems,
                2,
                self.player_data.levelexp as i32,
                nextexp as i32,
            );
        }

        for i in 0..MAX_EQPT {
            self.interface.profile.update_equipment_slot(
                systems,
                i,
                &self.player_data.equipment[i],
            );
        }

        if let Some(music) = &self.map.music[0].0 {
            if self.current_music != *music {
                self.current_music.clone_from(music);
                systems.audio.set_music(&format!("./audio/{}", music))?;
            }
        }

        self.finalized = true;
        Ok(())
    }

    pub fn move_map(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Socket,
        dir: Direction,
        buffer: &mut BufferTask,
    ) -> Result<()> {
        match dir {
            Direction::Down => self.map.map_pos.y -= 1,
            Direction::Left => self.map.map_pos.x -= 1,
            Direction::Right => self.map.map_pos.x += 1,
            Direction::Up => self.map.map_pos.y += 1,
        }

        let move_maps = match dir {
            Direction::Up => [(0, 2), (4, 1), (5, 3), (7, 0), (6, 4), (8, 5)],
            Direction::Left => [(0, 5), (2, 3), (7, 8), (1, 2), (4, 0), (6, 7)],
            Direction::Right => {
                [(0, 4), (2, 1), (7, 6), (3, 2), (5, 0), (8, 7)]
            }
            Direction::Down => [(0, 7), (4, 6), (5, 8), (2, 0), (1, 4), (3, 5)],
        };
        for (from, to) in move_maps {
            self.map.index[from].1 = to;
            self.map.map_attribute[from].1 = to;
            self.map.dir_block[from].1 = to;
            self.map.music[from].1 = to;
        }

        let load_maps = match dir {
            Direction::Up => [(1, 6), (2, 7), (3, 8)],
            Direction::Left => [(3, 1), (5, 4), (8, 6)],
            Direction::Right => [(1, 3), (4, 5), (6, 8)],
            Direction::Down => [(6, 1), (7, 2), (8, 3)],
        };
        for (from, to) in load_maps {
            let (mx, my) =
                get_map_loc(self.map.map_pos.x, self.map.map_pos.y, to);
            self.map.index[from].1 = to;
            self.map.map_attribute[from].1 = to;
            self.map.dir_block[from].1 = to;
            self.map.music[from].1 = to;

            buffer.add_task(BufferTaskEnum::LoadMap(
                mx,
                my,
                self.map.map_pos.group as u64,
            ));
            buffer.add_task(BufferTaskEnum::ApplyMap(
                mx,
                my,
                self.map.map_pos.group as u64,
                to,
            ));
            buffer.add_task(BufferTaskEnum::ApplyMapAttribute(
                mx,
                my,
                self.map.map_pos.group as u64,
                to,
            ));
            buffer.add_task(BufferTaskEnum::ApplyMapMusic(
                mx,
                my,
                self.map.map_pos.group as u64,
                to,
            ));
            buffer.add_task(BufferTaskEnum::UnloadMap(
                mx,
                my,
                self.map.map_pos.group as u64,
            ));
        }

        self.map.sort_map();

        if let Some(music) = &self.map.music[0].0 {
            if self.current_music != *music {
                self.current_music.clone_from(music);
                systems.audio.set_music(&format!("./audio/{}", music))?;
            }
        }

        update_camera(world, self, systems, socket)
    }

    pub fn handle_key_input(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Socket,
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

    pub fn spawn_item(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        pos: Position,
        cur_map: MapPosition,
        sprite: usize,
    ) -> Result<()> {
        let entity =
            MapItem::create(world, systems, sprite, pos, cur_map, None)?;
        self.mapitems.insert(entity);
        Ok(())
    }

    pub fn move_player(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        socket: &mut Socket,
        dir: &Direction,
    ) -> Result<()> {
        if let Some(myentity) = self.myentity {
            move_player(
                world,
                systems,
                socket,
                &myentity,
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
        socket: &mut Socket,
        seconds: f32,
    ) -> Result<()> {
        if let Some(myentity) = self.myentity {
            if world.get_or_err::<Attacking>(&myentity)?.0
                || world.get_or_err::<Movement>(&myentity)?.is_moving
            {
                return Ok(());
            }

            let pos = world.get_or_err::<Position>(&myentity)?;
            let dir = world.get_or_err::<Dir>(&myentity)?.0;

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

            let target_entity = world
                .query::<(&Position, &WorldEntityType)>()
                .iter()
                .find_map(|(entity, (pos, world_type))| {
                    if *pos == target_pos
                        && (*world_type == WorldEntityType::Npc
                            || *world_type == WorldEntityType::Player)
                    {
                        Some(Entity(entity))
                    } else {
                        None
                    }
                });

            if let Some(got_target) = target_entity {
                self.target.set_target(socket, systems, &got_target)?;
            }

            send_attack(socket, dir, target_entity)?;
            init_player_attack(world, systems, &myentity, seconds)?;
        }

        Ok(())
    }
}

pub fn update_player(
    world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Socket,
    content: &mut GameContent,
    buffer: &mut BufferTask,
    seconds: f32,
) -> Result<()> {
    let players = content.players.clone();
    for entity in players.iter() {
        if let Some(myentity) = content.myentity {
            if entity != &myentity {
                move_player(
                    world,
                    systems,
                    socket,
                    entity,
                    content,
                    MovementType::MovementBuffer,
                )?;
            }
        }

        process_player_movement(
            world, systems, socket, entity, content, buffer,
        )?;
        process_player_attack(world, systems, entity, seconds)?
    }
    Ok(())
}

pub fn update_npc(
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut GameContent,
    seconds: f32,
) -> Result<()> {
    let npcs = content.npcs.clone();
    for entity in npcs.iter() {
        if let Some(myentity) = content.myentity {
            if entity != &myentity {
                move_npc(world, systems, entity, MovementType::MovementBuffer)?;
            }
        }

        process_npc_movement(world, systems, entity)?;
        process_npc_attack(world, systems, entity, seconds)?;
    }
    Ok(())
}

pub fn finalize_entity(world: &mut World, systems: &mut SystemHolder) {
    for (_entity, (worldentitytype, sprite, finalized, hpbar, entitynamemap)) in
        world
            .query_mut::<(
                &WorldEntityType,
                &SpriteIndex,
                &mut Finalized,
                Option<&HPBar>,
                Option<&EntityNameMap>,
            )>()
            .into_iter()
            .filter(|(_, (_, _, finalized, _, _))| !finalized.0)
    {
        match worldentitytype {
            WorldEntityType::Player => {
                if let Some(hp_bar) = hpbar {
                    if let Some(nameindex) = entitynamemap {
                        player_finalized_data(
                            systems,
                            sprite.0,
                            nameindex.0,
                            hp_bar,
                        );
                        finalized.0 = true;
                    }
                }
            }
            WorldEntityType::Npc => {
                if let Some(hp_bar) = hpbar {
                    if let Some(nameindex) = entitynamemap {
                        npc_finalized_data(
                            systems,
                            sprite.0,
                            nameindex.0,
                            hp_bar,
                        );
                        finalized.0 = true;
                    }
                }
            }
            WorldEntityType::MapItem => {
                MapItem::finalized_data(systems, sprite.0);
                finalized.0 = true;
            }
            _ => {}
        }
    }
}

pub fn update_camera(
    world: &mut World,
    content: &mut GameContent,
    systems: &mut SystemHolder,
    socket: &mut Socket,
) -> Result<()> {
    let player_pos = if let Some(entity) = content.myentity {
        let pos_offset = world.get_or_err::<PositionOffset>(&entity)?;
        let pos = world.get_or_err::<Position>(&entity)?;
        (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32)
            + pos_offset.offset
    } else {
        Vec2::new(0.0, 0.0)
    };
    let adjust_pos = get_screen_center(&systems.size) - player_pos;
    content.camera.pos = adjust_pos;

    content.map.move_pos(systems, content.camera.pos);

    for (
        entity,
        (worldentitytype, sprite, pos, pos_offset, hp_bar, entitynamemap),
    ) in world
        .query_mut::<(
            &WorldEntityType,
            &SpriteIndex,
            &Position,
            &PositionOffset,
            Option<&mut HPBar>,
            Option<&mut EntityNameMap>,
        )>()
        .into_iter()
    {
        let is_target = if let Some(target) = content.target.entity {
            target.0 == entity
        } else {
            false
        };
        let is_my_entity = if let Some(myentity) = content.myentity {
            myentity.0 == entity
        } else {
            false
        };
        match worldentitytype {
            WorldEntityType::Player => {
                if let Some(hpbar) = hp_bar {
                    if let Some(namemap) = entitynamemap {
                        if is_target {
                            if !hpbar.visible {
                                hpbar.visible = true;
                                systems.gfx.set_visible(hpbar.bar_index, true);
                                systems.gfx.set_visible(hpbar.bg_index, true);
                            }
                        } else if hpbar.visible && !is_my_entity {
                            hpbar.visible = false;
                            systems.gfx.set_visible(hpbar.bar_index, false);
                            systems.gfx.set_visible(hpbar.bg_index, false);
                        }
                        update_player_position(
                            systems, content, socket, sprite.0, pos,
                            pos_offset, hpbar, namemap, is_target,
                        )?;
                    }
                }
            }
            WorldEntityType::Npc => {
                if let Some(hpbar) = hp_bar {
                    if let Some(namemap) = entitynamemap {
                        if is_target {
                            if !hpbar.visible {
                                hpbar.visible = true;
                                systems.gfx.set_visible(hpbar.bar_index, true);
                                systems.gfx.set_visible(hpbar.bg_index, true);
                            }
                        } else if hpbar.visible {
                            hpbar.visible = false;
                            systems.gfx.set_visible(hpbar.bar_index, false);
                            systems.gfx.set_visible(hpbar.bg_index, false);
                        }
                        update_npc_position(
                            systems, content, socket, sprite.0, pos,
                            pos_offset, hpbar, namemap, is_target,
                        )?;
                    }
                }
            }
            WorldEntityType::MapItem => {
                update_mapitem_position(
                    systems, content, sprite.0, pos, pos_offset,
                );
            }
            _ => {}
        }
    }
    Ok(())
}
