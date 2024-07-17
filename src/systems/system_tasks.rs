use graphics::*;
use indexmap::IndexMap;
use rustls::internal::msgs;
use std::collections::VecDeque;

use crate::{
    data_types::*, database::map::*, Content, MapAttributes, MapDirBlock,
    MessageChannel, Result, SystemHolder,
};

pub struct StoredData {
    pub map_data: IndexMap<String, MapData, ahash::RandomState>,
}

pub enum BufferTaskEnum {
    ApplyMap(MapPosition, Index),
}

pub struct BufferTask {
    pub task: VecDeque<BufferTaskEnum>,
    pub storage: StoredData,
    pub chatbuffer: ChatBufferTask,
}

impl Default for BufferTask {
    fn default() -> Self {
        BufferTask {
            task: VecDeque::new(),
            storage: StoredData {
                map_data: IndexMap::default(),
            },
            chatbuffer: ChatBufferTask::new(),
        }
    }
}

impl BufferTask {
    pub fn new() -> Self {
        BufferTask::default()
    }

    pub fn clear_buffer(&mut self) {
        self.task.clear();
        self.storage.map_data.clear();
    }

    pub fn process_buffer(
        &mut self,
        systems: &mut SystemHolder,
        content: &mut Content,
    ) -> Result<()> {
        self.chatbuffer.process_buffer(systems, content);

        if !self.task.is_empty() {
            let task_data = self.task.pop_front();

            if let Some(task) = task_data {
                match task {
                    BufferTaskEnum::ApplyMap(mappos, index) => {
                        load_map_data(systems, index, mappos)?;
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

#[derive(Default)]
pub struct ChatBufferTask {
    pub task: VecDeque<ChatTask>,
}

impl ChatBufferTask {
    pub fn new() -> Self {
        ChatBufferTask::default()
    }

    pub fn process_buffer(
        &mut self,
        systems: &mut SystemHolder,
        content: &mut Content,
    ) {
        if self.task.is_empty() || !content.game_content.finalized {
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
