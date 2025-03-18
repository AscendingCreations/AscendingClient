use graphics::*;
use mmap_bytey::MByteBuffer;

use crate::{
    Alert, GlobalKey, MyInstant, Result, World,
    content::Content,
    systems::{
        BufferTask, FADE_SWITCH_TO_GAME, FadeData, FadeType, Poller,
        SystemHolder, send_handshake, send_login_ok, send_tls_handshake,
    },
};

pub fn handle_ping(
    _socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    _data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let end_time = MyInstant::now();

    let elapse_time =
        end_time.duration_since(content.ping_start.0).as_millis() as u64;

    if systems.config.show_average_ping {
        let count =
            content.game_content.interface.average_ping_collection.len();
        if count > 0 {
            let sum: u64 = content
                .game_content
                .interface
                .average_ping_collection
                .iter()
                .sum();
            if sum > 0 {
                let average = sum / count as u64;
                systems.gfx.set_text(
                    &mut systems.renderer,
                    &content.game_content.interface.average_ping,
                    &format!("Av. Ping: {:?}", average),
                );
            }
            if count >= 20 {
                content
                    .game_content
                    .interface
                    .average_ping_collection
                    .pop_back();
            }
        }
        content
            .game_content
            .interface
            .average_ping_collection
            .push_front(elapse_time);
    }

    systems.gfx.set_text(
        &mut systems.renderer,
        &content.game_content.interface.ping_text,
        &format!("Ping: {:?}", elapse_time),
    );

    Ok(())
}

pub fn handle_handshake(
    socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let code = data.read::<String>()?;
    let handshake = data.read::<String>()?;
    systems.config.reconnect_code = code;
    systems.config.save_config("settings.toml");
    content.game_content.reconnect_count = 0;
    send_handshake(socket, handshake)
}

pub fn handle_loginok(
    socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _hour = data.read::<u32>()?;
    let _min = data.read::<u32>()?;

    systems.fade.init_fade(
        &mut systems.gfx,
        FadeType::In,
        FADE_SWITCH_TO_GAME,
        FadeData::None,
    );

    send_login_ok(socket, &systems.config.reconnect_code)
}

pub fn handle_myindex(
    _socket: &mut Poller,
    _world: &mut World,
    _systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let entity = data.read::<GlobalKey>()?;
    content.game_content.myentity = Some(entity);
    Ok(())
}

pub fn handle_tls_handshake(
    socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let code = data.read::<String>()?;
    let handshake = data.read::<String>()?;
    systems.config.reconnect_code = code;
    systems.config.save_config("settings.toml");
    content.game_content.reconnect_count = 0;
    send_tls_handshake(socket, handshake)
}

pub fn handle_clear_data(
    _socket: &mut Poller,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _ = data.read::<u32>()?;

    systems
        .gfx
        .set_color(&systems.fade.f_image, Color::rgba(0, 0, 0, 255));
    systems.fade.f_alpha = 255;

    content.game_content.clear_data(world, systems)?;
    content.game_content.show(systems);
    Ok(())
}

pub fn handle_playitemsfx(
    _socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let index = data.read::<u16>()?;

    if let Some(sfx_name) = &systems.base.item[index as usize].sound_index {
        let volume = systems.config.sfx_volume as f32 * 0.01;
        systems
            .audio
            .play_effect(format!("./audio/{}", sfx_name), volume)?;
    }

    Ok(())
}
