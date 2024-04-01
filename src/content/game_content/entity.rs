use crate::{values::*, ClientError, Direction, Result};
use bytey::{ByteBufferError, ByteBufferRead, ByteBufferWrite};
use core::any::type_name;
use educe::Educe;
use graphics::*;
use hecs::{EntityRef, MissingComponent, World};
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::{
    backtrace::Backtrace,
    ops::{Deref, DerefMut},
};

pub enum MovementType {
    MovementBuffer,
    Manual(u8, Option<Position>),
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Finalized(pub bool);

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    ByteBufferRead,
    ByteBufferWrite,
)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
    pub group: i32,
}

impl MapPosition {
    pub fn new(x: i32, y: i32, group: i32) -> Self {
        MapPosition { x, y, group }
    }
}

#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, ByteBufferRead, ByteBufferWrite,
)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map: MapPosition,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PositionOffset {
    pub offset: Vec2,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct HPBar {
    pub visible: bool,
    pub bg_index: usize,
    pub bar_index: usize,
}

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct Dir(pub u8);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct Level(pub i32);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct LastMoveFrame(pub usize);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct SpriteIndex(pub usize);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct SpriteImage(pub u8);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct Attacking(pub bool);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct AttackTimer(pub f32);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct AttackFrame {
    pub frame: usize,
    pub timer: f32,
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Default, ByteBufferRead, ByteBufferWrite,
)]
pub struct Physical {
    pub damage: u32,
    pub defense: u32,
}

#[derive(
    Educe, Debug, Copy, Clone, PartialEq, Eq, ByteBufferRead, ByteBufferWrite,
)]
#[educe(Default)]
pub struct Vitals {
    #[educe(Default = [25, 2, 100])]
    pub vital: [i32; VITALS_MAX],
    #[educe(Default = [25, 2, 100])]
    pub vitalmax: [i32; VITALS_MAX],
    #[educe(Default = [0; VITALS_MAX])]
    pub vitalbuffs: [i32; VITALS_MAX],
    #[educe(Default = [0; VITALS_MAX])]
    pub regens: [u32; VITALS_MAX],
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Movement {
    pub is_moving: bool,
    pub move_direction: Direction,
    pub move_timer: f32,
    pub move_offset: f32,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct EndMovement(pub Position);

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct MovementData {
    pub end_pos: Position,
    pub dir: u8,
}

#[derive(Debug, Clone, Default)]
pub struct MovementBuffer {
    pub data: VecDeque<MovementData>,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Hidden(pub bool);

#[derive(Debug, Default)]
pub struct EntityName(pub String);

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Educe,
    ByteBufferWrite,
    ByteBufferRead,
)]
#[educe(Default)]
pub struct Item {
    pub num: u32,
    pub val: u16,
    #[educe(Default = 1)]
    pub level: u8,
    pub data: [i16; 5],
}

#[derive(
    PartialEq,
    Eq,
    Clone,
    Debug,
    Educe,
    Deserialize,
    Serialize,
    ByteBufferRead,
    ByteBufferWrite,
)]
#[educe(Default)]
pub struct Equipment {
    #[educe(Default = (0..MAX_EQPT).map(|_| Item::default()).collect())]
    pub items: Vec<Item>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum WorldEntityType {
    #[default]
    None,
    Player,
    Npc,
    MapItem,
    Map,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    ByteBufferRead,
    ByteBufferWrite,
)]
pub enum DeathType {
    #[default]
    Alive,
    Spirit,
    Dead,
    UnSpawned,
    Spawning,
}

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub enum UserAccess {
    #[default]
    None,
    Monitor,
    Admin,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Default,
    ByteBufferRead,
    ByteBufferWrite,
)]
pub enum NpcMode {
    None,
    #[default]
    Normal,
    Pet,
    Summon,
    Boss,
}

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct NpcIndex(pub u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Entity(pub hecs::Entity);

impl ByteBufferWrite for Entity {
    fn write_to_buffer(
        &self,
        buffer: &mut bytey::ByteBuffer,
    ) -> bytey::Result<()> {
        self.0.to_bits().write_to_buffer(buffer)
    }

    fn write_to_buffer_le(
        &self,
        buffer: &mut bytey::ByteBuffer,
    ) -> bytey::Result<()> {
        self.0.to_bits().write_to_buffer_le(buffer)
    }

    fn write_to_buffer_be(
        &self,
        buffer: &mut bytey::ByteBuffer,
    ) -> bytey::Result<()> {
        self.0.to_bits().write_to_buffer_be(buffer)
    }
}

impl ByteBufferRead for Entity {
    fn read_from_buffer(buffer: &mut bytey::ByteBuffer) -> bytey::Result<Self>
    where
        Self: Sized,
    {
        Ok(Entity(
            hecs::Entity::from_bits(buffer.read::<u64>()?).ok_or(ByteBufferError::OtherError {
                error: "Bits could not be converted to hecs Entity. Is your Struct wrong?"
                    .to_owned(),
            })?,
        ))
    }

    fn read_from_buffer_le(
        buffer: &mut bytey::ByteBuffer,
    ) -> bytey::Result<Self>
    where
        Self: Sized,
    {
        Ok(Entity(
            hecs::Entity::from_bits(buffer.read_le::<u64>()?).ok_or(
                ByteBufferError::OtherError {
                    error: "Bits could not be converted to hecs Entity. Is your Struct wrong?"
                        .to_owned(),
                },
            )?,
        ))
    }

    fn read_from_buffer_be(
        buffer: &mut bytey::ByteBuffer,
    ) -> bytey::Result<Self>
    where
        Self: Sized,
    {
        Ok(Entity(
            hecs::Entity::from_bits(buffer.read_be::<u64>()?).ok_or(
                ByteBufferError::OtherError {
                    error: "Bits could not be converted to hecs Entity. Is your Struct wrong?"
                        .to_owned(),
                },
            )?,
        ))
    }
}

pub trait WorldExtras {
    fn get_or_default<T>(&self, entity: &Entity) -> T
    where
        T: Default + Send + Sync + Copy + 'static;
    fn cloned_get_or_default<T>(&self, entity: &Entity) -> T
    where
        T: Default + Send + Sync + Clone + 'static;
    fn get_or_panic<T>(&self, entity: &Entity) -> T
    where
        T: Send + Sync + Copy + 'static;
    fn cloned_get_or_panic<T>(&self, entity: &Entity) -> T
    where
        T: Send + Sync + Clone + 'static;
    fn get_or_err<T>(&self, entity: &Entity) -> Result<T>
    where
        T: Send + Sync + Copy + 'static;
    fn cloned_get_or_err<T>(&self, entity: &Entity) -> Result<T>
    where
        T: Send + Sync + Clone + 'static;
}

pub trait WorldEntityExtras {
    fn get_or_default<T>(&self) -> T
    where
        T: Default + Send + Sync + Copy + 'static;
    fn cloned_get_or_default<T>(&self) -> T
    where
        T: Default + Send + Sync + Clone + 'static;
    fn get_or_panic<T>(&self) -> T
    where
        T: Send + Sync + Copy + 'static;
    fn cloned_get_or_panic<T>(&self) -> T
    where
        T: Send + Sync + Clone + 'static;
    fn get_or_err<T>(&self) -> Result<T>
    where
        T: Send + Sync + Copy + 'static;
    fn cloned_get_or_err<T>(&self) -> Result<T>
    where
        T: Send + Sync + Clone + 'static;
}

impl WorldEntityExtras for EntityRef<'_> {
    fn get_or_default<T>(&self) -> T
    where
        T: Default + Send + Sync + Copy + 'static,
    {
        self.get::<&T>().map(|t| *t).unwrap_or_default()
    }

    fn cloned_get_or_default<T>(&self) -> T
    where
        T: Default + Send + Sync + Clone + 'static,
    {
        self.get::<&T>().map(|t| (*t).clone()).unwrap_or_default()
    }

    fn get_or_panic<T>(&self) -> T
    where
        T: Send + Sync + Copy + 'static,
    {
        match self.get::<&T>() {
            Some(t) => *t,
            None => {
                error!("Component: {} is missing.", type_name::<T>());
                panic!("Component: {} is missing.", type_name::<T>());
            }
        }
    }

    fn cloned_get_or_panic<T>(&self) -> T
    where
        T: Send + Sync + Clone + 'static,
    {
        match self.get::<&T>() {
            Some(t) => (*t).clone(),
            None => {
                error!("Component: {} is missing.", type_name::<T>());
                panic!("Component: {} is missing.", type_name::<T>());
            }
        }
    }

    fn get_or_err<T>(&self) -> Result<T>
    where
        T: Send + Sync + Copy + 'static,
    {
        match self.get::<&T>().map(|t| *t) {
            Some(t) => Ok(t),
            None => {
                let e = ClientError::HecsComponent {
                    error: hecs::ComponentError::MissingComponent(
                        MissingComponent::new::<T>(),
                    ),
                    backtrace: Box::new(Backtrace::capture()),
                };

                warn!("Component Err: {:?}", e);
                Err(e)
            }
        }
    }

    fn cloned_get_or_err<T>(&self) -> Result<T>
    where
        T: Send + Sync + Clone + 'static,
    {
        match self.get::<&T>().map(|t| (*t).clone()) {
            Some(t) => Ok(t),
            None => {
                let e = ClientError::HecsComponent {
                    error: hecs::ComponentError::MissingComponent(
                        MissingComponent::new::<T>(),
                    ),
                    backtrace: Box::new(Backtrace::capture()),
                };

                warn!("Component Err: {:?}", e);
                Err(e)
            }
        }
    }
}

impl WorldExtras for World {
    fn get_or_default<T>(&self, entity: &Entity) -> T
    where
        T: Default + Send + Sync + Copy + 'static,
    {
        self.get::<&T>(entity.0).map(|t| *t).unwrap_or_default()
    }

    fn cloned_get_or_default<T>(&self, entity: &Entity) -> T
    where
        T: Default + Send + Sync + Clone + 'static,
    {
        self.get::<&T>(entity.0)
            .map(|t| (*t).clone())
            .unwrap_or_default()
    }

    fn get_or_panic<T>(&self, entity: &Entity) -> T
    where
        T: Send + Sync + Copy + 'static,
    {
        match self.get::<&T>(entity.0) {
            Ok(t) => *t,
            Err(e) => {
                error!("Component error: {:?}", e);
                panic!("Component error: {:?}", e);
            }
        }
    }

    fn cloned_get_or_panic<T>(&self, entity: &Entity) -> T
    where
        T: Send + Sync + Clone + 'static,
    {
        match self.get::<&T>(entity.0) {
            Ok(t) => (*t).clone(),
            Err(e) => {
                error!("Component error: {:?}", e);
                panic!("Component error: {:?}", e);
            }
        }
    }

    fn get_or_err<T>(&self, entity: &Entity) -> Result<T>
    where
        T: Send + Sync + Copy + 'static,
    {
        match self.get::<&T>(entity.0).map(|t| *t) {
            Ok(t) => Ok(t),
            Err(e) => {
                warn!("Component Err: {:?}", e);
                Err(ClientError::HecsComponent {
                    error: e,
                    backtrace: Box::new(Backtrace::capture()),
                })
            }
        }
    }

    fn cloned_get_or_err<T>(&self, entity: &Entity) -> Result<T>
    where
        T: Send + Sync + Clone + 'static,
    {
        match self.get::<&T>(entity.0).map(|t| (*t).clone()) {
            Ok(t) => Ok(t),
            Err(e) => {
                warn!("Component Err: {:?}", e);
                Err(ClientError::HecsComponent {
                    error: e,
                    backtrace: Box::new(Backtrace::capture()),
                })
            }
        }
    }
}
