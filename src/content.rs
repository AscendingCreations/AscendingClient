use crate::{Config, Result, SystemHolder, World, data_types::*, database::*};

pub mod game_content;
pub mod inputs;
pub mod menu_content;
pub mod resource;

pub use game_content::*;
use graphics::MapRenderer;
pub use inputs::*;
use log::info;
pub use menu_content::*;
pub use resource::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Menu,
    Game,
}

pub struct Content {
    pub menu_content: MenuContent,
    pub game_content: GameContent,
    pub content_type: ContentType,
    pub ping_start: MyInstant,
}

impl Content {
    pub fn new(
        world: &mut World,
        systems: &mut SystemHolder,
        map_renderer: &mut MapRenderer,
    ) -> Result<Self> {
        let mut content = Content {
            content_type: ContentType::Menu,
            menu_content: MenuContent::new(systems),
            game_content: GameContent::new(systems),
            ping_start: MyInstant::now(),
        };
        content.menu_content.show(systems);
        content.game_content.hide(world, systems, map_renderer)?;

        Ok(content)
    }

    pub fn switch_content(
        &mut self,
        world: &mut World,
        systems: &mut SystemHolder,
        map_renderer: &mut MapRenderer,
        contenttype: ContentType,
    ) -> Result<()> {
        if self.content_type == contenttype {
            return Ok(());
        }

        match self.content_type {
            ContentType::Game => {
                self.game_content.hide(world, systems, map_renderer)?;
            }
            ContentType::Menu => {
                self.menu_content.hide(systems);
            }
        }
        self.content_type = contenttype;

        match self.content_type {
            ContentType::Game => {
                self.game_content.show(systems);
            }
            ContentType::Menu => {
                self.menu_content.show(systems);
            }
        }

        Ok(())
    }
}
