use graphics::*;
use indexmap::IndexMap;
use rustls::internal::msgs;
use std::collections::VecDeque;

use crate::{
    database::map::*, Content, HPBar, MapAttributes, MapDirBlock,
    MessageChannel, Result, SystemHolder,
};

pub struct StoredData {
    pub map_data: IndexMap<String, MapData>,
}

pub enum BufferTaskEnum {
    LoadMap(i32, i32, u64),
    ApplyMap(i32, i32, u64, usize),
    ApplyMapAttribute(i32, i32, u64, usize),
    ApplyMapMusic(i32, i32, u64, usize),
    ApplyMapDirBlock(i32, i32, u64, usize),
    UnloadMap(i32, i32, u64),
}

pub struct BufferTask {
    pub task: VecDeque<BufferTaskEnum>,
    pub storage: StoredData,
    pub chatbuffer: ChatBufferTask,
}

impl BufferTask {
    pub fn new() -> Self {
        BufferTask {
            task: VecDeque::new(),
            storage: StoredData {
                map_data: IndexMap::new(),
            },
            chatbuffer: ChatBufferTask::new(),
        }
    }

    pub fn process_buffer(
        &mut self,
        systems: &mut SystemHolder,
        content: &mut Content,
    ) -> Result<()> {
        self.chatbuffer.process_buffer(systems, content);
        if self.task.is_empty() {
            return Ok(());
        }

        let task_data = self.task.pop_front();
        if let Some(task) = task_data {
            match task {
                BufferTaskEnum::ApplyMap(mx, my, mg, map_index) => {
                    let key = format!("{}_{}_{}", mx, my, mg);
                    if let Some(mapdata) = self.storage.map_data.get(&key) {
                        load_map_data(
                            systems,
                            mapdata,
                            content.game_content.map.index[map_index].0,
                        );
                    }
                }
                BufferTaskEnum::ApplyMapAttribute(mx, my, mg, map_index) => {
                    let key = format!("{}_{}_{}", mx, my, mg);
                    if let Some(mapdata) = self.storage.map_data.get(&key) {
                        content.game_content.map.map_attribute[map_index].0 =
                            MapAttributes {
                                attribute: mapdata.attribute.clone(),
                            };
                    }
                }
                BufferTaskEnum::ApplyMapDirBlock(mx, my, mg, map_index) => {
                    let key = format!("{}_{}_{}", mx, my, mg);
                    if let Some(mapdata) = self.storage.map_data.get(&key) {
                        content.game_content.map.dir_block[map_index].0 =
                            MapDirBlock {
                                dir: mapdata.dir_block.clone(),
                            };
                    }
                }
                BufferTaskEnum::ApplyMapMusic(mx, my, mg, map_index) => {
                    let key = format!("{}_{}_{}", mx, my, mg);
                    if let Some(mapdata) = self.storage.map_data.get(&key) {
                        content.game_content.map.music[map_index].0 =
                            mapdata.music.clone();
                    }
                }
                BufferTaskEnum::LoadMap(mx, my, mg) => {
                    let key = format!("{}_{}_{}", mx, my, mg);
                    let map_data = load_file(mx, my, mg)?;
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
        Ok(())
    }

    pub fn add_task(&mut self, task: BufferTaskEnum) {
        self.task.push_back(task);
    }
}

pub struct ChatTask {
    msg: (String, Color),
    header_msg: Option<(String, Color)>,
    channel: MessageChannel,
}

impl ChatTask {
    pub fn new(
        msg: (String, Color),
        header_msg: Option<(String, Color)>,
        channel: MessageChannel,
    ) -> Self {
        ChatTask {
            msg,
            header_msg,
            channel,
        }
    }
}

pub struct ChatBufferTask {
    pub task: VecDeque<ChatTask>,
}

impl ChatBufferTask {
    pub fn new() -> Self {
        ChatBufferTask {
            task: VecDeque::new(),
        }
    }

    pub fn process_buffer(
        &mut self,
        systems: &mut SystemHolder,
        content: &mut Content,
    ) {
        if self.task.is_empty() {
            return;
        }
        if !content.game_content.finalized {
            return;
        }

        let task_data = self.task.pop_front();
        if let Some(task) = task_data {
            content.game_content.interface.chatbox.add_chat(
                systems,
                task.msg,
                task.header_msg,
                task.channel,
            );
        }
    }

    pub fn add_task(&mut self, task: ChatTask) {
        self.task.push_back(task);
    }
}
