use indexmap::IndexMap;
use std::collections::VecDeque;

use crate::{database::map::*, Content, DrawSetting, MapAttributes};

pub struct StoredData {
    pub map_data: IndexMap<String, MapData>,
}

pub enum BufferTaskEnum {
    LoadMap(i32, i32, u64),
    ApplyMap(i32, i32, u64, usize),
    ApplyMapAttribute(i32, i32, u64, usize),
    UnloadMap(i32, i32, u64),
}

pub struct BufferTask {
    pub task: VecDeque<BufferTaskEnum>,
    pub storage: StoredData,
}

impl BufferTask {
    pub fn new() -> Self {
        BufferTask {
            task: VecDeque::new(),
            storage: StoredData {
                map_data: IndexMap::new(),
            },
        }
    }

    pub fn process_buffer(&mut self, systems: &mut DrawSetting, content: &mut Content) {
        if self.task.is_empty() {
            return;
        }

        if let Some(task) = self.task.pop_front() {
            match task {
                BufferTaskEnum::ApplyMap(mx, my, mg, map_index) => {
                    let key = format!("{}_{}_{}", mx, my, mg);
                    if let Some(mapdata) = self.storage.map_data.get(&key) {
                        load_map_data(systems, mapdata, content.game_content.map.index[map_index].0);
                    }
                }
                BufferTaskEnum::ApplyMapAttribute(mx, my, mg, map_index) => {
                    let key = format!("{}_{}_{}", mx, my, mg);
                    if let Some(mapdata) = self.storage.map_data.get(&key) {
                        content.game_content.map.map_attribute[map_index].0 = MapAttributes { attribute: mapdata.attribute.clone() };
                    }
                }
                BufferTaskEnum::LoadMap(mx, my, mg) => {
                    let key = format!("{}_{}_{}", mx, my, mg);
                    let map_data = load_file(mx, my, mg);
                    self.storage.map_data.insert(key, map_data);
                }
                BufferTaskEnum::UnloadMap(mx, my, mg) => {
                    let key = format!("{}_{}_{}", mx, my, mg);
                    if self.storage.map_data.contains_key(&key) {
                        self.storage.map_data.shift_remove(&key);
                    }
                }
            }
        }
    }

    pub fn add_task(&mut self, task: BufferTaskEnum) {
        self.task.push_back(task);
    }
}