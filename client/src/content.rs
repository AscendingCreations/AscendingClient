use crate::DrawSetting;
use hecs::World;

pub mod game_content;
pub mod menu_content;

pub use game_content::*;
pub use menu_content::*;

pub enum ContentHolder {
    Menu(MenuContent),
    Game(GameContent),
}

#[derive(PartialEq, Eq)]
pub enum ContentType {
    Menu,
    Game,
}

pub struct Content {
    pub holder: ContentHolder,
    content_type: ContentType,
}

impl Content {
    pub fn new(world: &mut World, systems: &mut DrawSetting) -> Self {
        Content {
            holder: ContentHolder::Menu(MenuContent::new(world, systems)),
            content_type: ContentType::Menu,
        }
    }

    pub fn switch_content(&mut self, world: &mut World, systems: &mut DrawSetting, contenttype: ContentType) {
        if self.content_type == contenttype {
            return;
        }
        
        match &mut self.holder {
            ContentHolder::Game(data) => {
                data.unload(world, systems);
            }
            ContentHolder::Menu(data) => {
                data.unload(world, systems);
            }
        }

        self.content_type = contenttype;
        match self.content_type {
            ContentType::Game => {
                self.holder = ContentHolder::Game(GameContent::new(world, systems));
            }
            ContentType::Menu => {
                self.holder = ContentHolder::Menu(MenuContent::new(world, systems));
            }
        }

        println!("Gfx Collection: {:?}", systems.gfx.count_collection())
    }
}