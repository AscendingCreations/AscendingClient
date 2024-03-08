use graphics::*;
use hecs::{EntityRef, World};
use serde::{Deserialize, Serialize};
use core::any::type_name;
use bytey::{ByteBufferRead, ByteBufferWrite, ByteBufferError};

use crate::Direction;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, ByteBufferRead, ByteBufferWrite)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
    pub group: i32,
}

impl MapPosition {
    pub fn new(x: i32, y: i32, group: i32) -> Self {
        MapPosition {
            x,
            y,
            group,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, ByteBufferRead, ByteBufferWrite)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map: MapPosition,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PositionOffset {
    pub offset: Vec2,
}

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct Dir(pub u8);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct LastMoveFrame(pub usize);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct Sprite(pub usize);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct Attacking(pub bool);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct AttackTimer(pub f32);

#[derive(Copy, Clone, Debug, Default, ByteBufferRead, ByteBufferWrite)]
pub struct AttackFrame {
    pub frame: usize,
    pub timer: f32,
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

#[derive(Copy, Clone, Debug, Default)]
pub enum WorldEntityType {
    #[default]
    None,
    Player,
    Npc,
    MapItem,
    Map,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Entity(pub hecs::Entity);

impl ByteBufferWrite for Entity {
    fn write_to_buffer(&self, buffer: &mut bytey::ByteBuffer) -> bytey::Result<()> {
        self.0.to_bits().write_to_buffer(buffer)
    }

    fn write_to_buffer_le(&self, buffer: &mut bytey::ByteBuffer) -> bytey::Result<()> {
        self.0.to_bits().write_to_buffer_le(buffer)
    }

    fn write_to_buffer_be(&self, buffer: &mut bytey::ByteBuffer) -> bytey::Result<()> {
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
                error: "Bits could nto be converted to hecs Entity. Is your Struct wrong?"
                    .to_owned(),
            })?,
        ))
    }

    fn read_from_buffer_le(buffer: &mut bytey::ByteBuffer) -> bytey::Result<Self>
    where
        Self: Sized,
    {
        Ok(Entity(
            hecs::Entity::from_bits(buffer.read_le::<u64>()?).ok_or(
                ByteBufferError::OtherError {
                    error: "Bits could nto be converted to hecs Entity. Is your Struct wrong?"
                        .to_owned(),
                },
            )?,
        ))
    }

    fn read_from_buffer_be(buffer: &mut bytey::ByteBuffer) -> bytey::Result<Self>
    where
        Self: Sized,
    {
        Ok(Entity(
            hecs::Entity::from_bits(buffer.read_be::<u64>()?).ok_or(
                ByteBufferError::OtherError {
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
        T: Default + Send + Sync + Copy + 'static;
    fn get_or_panic<T>(&self, entity: &Entity) -> T
    where
        T: Send + Sync + Copy + 'static;
    fn cloned_get_or_panic<T>(&self, entity: &Entity) -> T
    where
        T: Send + Sync + Copy + 'static;
}

pub trait WorldEntityExtras {
    fn get_or_default<T>(&self) -> T
    where
        T: Default + Send + Sync + Copy + 'static;
    fn cloned_get_or_default<T>(&self) -> T
    where
        T: Default + Send + Sync + Copy + 'static;
    fn get_or_panic<T>(&self) -> T
    where
        T: Send + Sync + Copy + 'static;
    fn cloned_get_or_panic<T>(&self) -> T
    where
        T: Send + Sync + Copy + 'static;
}

impl WorldEntityExtras for EntityRef<'_> {
    fn get_or_default<T>(&self) -> T
    where
        T: Default + Send + Sync + Copy + 'static,
    {
        match self.get::<&T>() {
            Some(t) => *t,
            None => T::default(),
        }
    }

    fn cloned_get_or_default<T>(&self) -> T
    where
        T: Default + Send + Sync + Copy + 'static,
    {
        match self.get::<&T>() {
            Some(t) => (*t).clone(),
            None => T::default(),
        }
    }

    fn get_or_panic<T>(&self) -> T
    where
        T: Send + Sync + Copy + 'static,
    {
        match self.get::<&T>() {
            Some(t) => *t,
            None => panic!("Component: {} is missing.", type_name::<T>()),
        }
    }

    fn cloned_get_or_panic<T>(&self) -> T
    where
        T: Send + Sync + Copy + 'static,
    {
        match self.get::<&T>() {
            Some(t) => (*t).clone(),
            None => panic!("Component: {} is missing.", type_name::<T>()),
        }
    }
}

impl WorldExtras for World {
    fn get_or_default<T>(&self, entity: &Entity) -> T
    where
        T: Default + Send + Sync + Copy + 'static,
    {
        match self.get::<&T>(entity.0) {
            Ok(t) => *t,
            Err(_) => T::default(),
        }
    }

    fn cloned_get_or_default<T>(&self, entity: &Entity) -> T
    where
        T: Default + Send + Sync + Copy + 'static,
    {
        match self.get::<&T>(entity.0) {
            Ok(t) => (*t).clone(),
            Err(_) => T::default(),
        }
    }

    fn get_or_panic<T>(&self, entity: &Entity) -> T
    where
        T: Send + Sync + Copy + 'static,
    {
        match self.get::<&T>(entity.0) {
            Ok(t) => *t,
            Err(e) => panic!("Component error: {:?}", e),
        }
    }

    fn cloned_get_or_panic<T>(&self, entity: &Entity) -> T
    where
        T: Send + Sync + Copy + 'static,
    {
        match self.get::<&T>(entity.0) {
            Ok(t) => (*t).clone(),
            Err(e) => panic!("Component error: {:?}", e),
        }
    }
}