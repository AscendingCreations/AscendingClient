use graphics::*;

use input::Key;
use winit::{event_loop::ActiveEventLoop, keyboard::NamedKey};

use crate::{
    Alert, Direction, EntityKind, Result, SystemHolder, Tooltip,
    content::{menu_content::content_input::*, *},
    socket::*,
};

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum MouseInputType {
    MouseLeftDown,
    MouseDoubleLeftDown,
    MouseLeftDownMove,
    MouseMove,
    MouseRelease,
}

#[allow(clippy::too_many_arguments)]
pub fn handle_mouse_input(
    world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Poller,
    elwt: &ActiveEventLoop,
    input_type: MouseInputType,
    mouse_pos: &Vec2,
    content: &mut Content,
    alert: &mut Alert,
    tooltip: &mut Tooltip,
) -> Result<()> {
    // We convert the mouse position to render position as the y pos increase upward
    let screen_pos = Vec2::new(mouse_pos.x, systems.size.height - mouse_pos.y);

    tooltip.check_tooltip(systems, screen_pos);

    match content.content_type {
        ContentType::Game => {
            GameContent::mouse_input(
                content, world, systems, socket, elwt, alert, tooltip,
                input_type, screen_pos,
            )?;
        }
        ContentType::Menu => {
            MenuContent::mouse_input(
                content, world, systems, socket, elwt, alert, tooltip,
                input_type, screen_pos,
            )?;
        }
    }
    Ok(())
}

pub fn handle_key_input(
    world: &mut World,
    systems: &mut SystemHolder,
    socket: &mut Poller,
    content: &mut Content,
    alert: &mut Alert,
    key: &Key,
    pressed: bool,
) -> Result<()> {
    match content.content_type {
        ContentType::Game => {
            GameContent::key_input(
                content, world, systems, socket, alert, key, pressed,
            )?;
        }
        ContentType::Menu => {
            MenuContent::key_input(
                content, world, systems, socket, alert, key, pressed,
            );
        }
    }

    if pressed {
        #[allow(clippy::single_match)]
        match key {
            Key::Named(NamedKey::F1) => systems.gfx.count_collection(),
            Key::Named(NamedKey::F3) => {
                info!("World NPC Count: {}", count_npc(world));
                info!("World Player Count: {}", count_player(world));
                info!("World MapItems Count: {}", count_mapitems(world));
            }
            Key::Named(NamedKey::F4) => {
                info!(
                    "Storage Len Size NPC Count: {}",
                    content.game_content.npcs.borrow().len()
                );
                info!(
                    "Storage Len Size Player Count: {}",
                    content.game_content.players.borrow().len()
                );
                info!(
                    "Storage Len Size MapItems Count: {}",
                    content.game_content.mapitems.borrow().len()
                );
            }
            _ => {}
        }
    }

    Ok(())
}

fn count_npc(world: &mut World) -> usize {
    world
        .kinds
        .iter()
        .filter(|(_, entity_kind)| **entity_kind == EntityKind::Npc)
        .count()
}

fn count_player(world: &mut World) -> usize {
    world
        .kinds
        .iter()
        .filter(|(_, entity_kind)| **entity_kind == EntityKind::Player)
        .count()
}

fn count_mapitems(world: &mut World) -> usize {
    world
        .kinds
        .iter()
        .filter(|(_, entity_kind)| **entity_kind == EntityKind::MapItem)
        .count()
}
