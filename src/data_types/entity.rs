use crate::{data_types::*, socket::*, ClientError, Direction, Result};
use core::any::type_name;
use educe::Educe;
use graphics::*;
use hecs::{EntityRef, MissingComponent, World};
use log::{error, warn};
use serde::{Deserialize, Serialize};
use speedy::{Endianness, Readable, Writable};
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
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
    Hash,
)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
    pub group: i32,
}

impl MapPosition {
    pub fn checkdistance(&self, target: MapPosition) -> i32 {
        if self.group != target.group {
            return 2;
        }

        let x = self.x - target.x;
        let y = self.y - target.y;

        if x == 0 {
            return y.abs();
        }

        if y == 0 {
            return x.abs();
        }

        x.abs() + y.abs() - 1
    }
}

impl MapPosition {
    pub fn new(x: i32, y: i32, group: i32) -> Self {
        MapPosition { x, y, group }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Readable,
    Writable,
    MByteBufferRead,
    MByteBufferWrite,
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
    pub bg_index: GfxType,
    pub bar_index: GfxType,
}

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct Dir(pub u8);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct Level(pub i32);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct LastMoveFrame(pub usize);

#[derive(Copy, Clone, Debug, Default)]
pub struct SpriteIndex(pub GfxType);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct SpriteImage(pub u8);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct Attacking(pub bool);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct AttackTimer(pub f32);

#[derive(Copy, Clone, Debug, Default)]
pub struct EntityLight(pub Option<Index>);

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct AttackFrame {
    pub frame: usize,
    pub timer: f32,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Default,
    MByteBufferRead,
    MByteBufferWrite,
)]
pub struct Physical {
    pub damage: u32,
    pub defense: u32,
}

#[derive(
    Educe, Debug, Copy, Clone, PartialEq, Eq, MByteBufferRead, MByteBufferWrite,
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

#[derive(Debug, Clone, Default)]
pub struct EntityName(pub String);

#[derive(Debug, Clone, Copy, Default)]
pub struct EntityNameMap(pub GfxType);

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Educe,
    MByteBufferRead,
    MByteBufferWrite,
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
    MByteBufferRead,
    MByteBufferWrite,
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
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum DeathType {
    #[default]
    Alive,
    Spirit,
    Dead,
    UnSpawned,
    Spawning,
}

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
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
    MByteBufferRead,
    MByteBufferWrite,
)]
pub enum NpcMode {
    None,
    #[default]
    Normal,
    Pet,
    Summon,
    Boss,
}

#[derive(Copy, Clone, Debug, Default, MByteBufferRead, MByteBufferWrite)]
pub struct NpcIndex(pub u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Entity(pub hecs::Entity);

impl MByteBufferWrite for Entity {
    fn write_to_mbuffer(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        self.0.to_bits().write_to_mbuffer(buffer)
    }

    fn write_to_mbuffer_le(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        self.0.to_bits().write_to_mbuffer_le(buffer)
    }

    fn write_to_mbuffer_be(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        self.0.to_bits().write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferWrite for &Entity {
    fn write_to_mbuffer(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        self.0.to_bits().write_to_mbuffer(buffer)
    }

    fn write_to_mbuffer_le(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        self.0.to_bits().write_to_mbuffer_le(buffer)
    }

    fn write_to_mbuffer_be(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        self.0.to_bits().write_to_mbuffer_be(buffer)
    }
}

impl MByteBufferRead for Entity {
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> mmap_bytey::Result<Self>
    where
        Self: Sized,
    {
        Ok(Entity(
            hecs::Entity::from_bits(buffer.read::<u64>()?).ok_or(MByteBufferError::OtherError {
                error: "Bits could nto be converted to hecs Entity. Is your Struct wrong?"
                    .to_owned(),
            })?,
        ))
    }

    fn read_from_mbuffer_le(
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<Self>
    where
        Self: Sized,
    {
        Ok(Entity(
            hecs::Entity::from_bits(buffer.read_le::<u64>()?).ok_or(
                MByteBufferError::OtherError {
                    error: "Bits could nto be converted to hecs Entity. Is your Struct wrong?"
                        .to_owned(),
                },
            )?,
        ))
    }

    fn read_from_mbuffer_be(
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<Self>
    where
        Self: Sized,
    {
        Ok(Entity(
            hecs::Entity::from_bits(buffer.read_be::<u64>()?).ok_or(
                MByteBufferError::OtherError {
                    error: "Bits could nto be converted to hecs Entity. Is your Struct wrong?"
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
