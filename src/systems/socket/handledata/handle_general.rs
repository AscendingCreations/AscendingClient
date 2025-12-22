use graphics::*;
use mmap_bytey::MByteBuffer;

use crate::{
    Alert, GlobalKey, MyInstant, Result, World,
    content::Content,
    systems::{
        BufferTask, FADE_SWITCH_TO_GAME, FadeData, FadeType, Poller,
        SystemHolder, mapper::PacketPasser, send_handshake, send_login_ok,
        send_tls_handshake,
    },
};

pub fn handle_ping(
    _data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let end_time = MyInstant::now();

    let elapse_time = end_time
        .duration_since(passer.content.ping_start.0)
        .as_millis() as u64;

    if passer.systems.config.show_average_ping {
        let count = passer
            .content
            .game_content
            .interface
            .average_ping_collection
            .len();
        if count > 0 {
            let sum: u64 = passer
                .content
                .game_content
                .interface
                .average_ping_collection
                .iter()
                .sum();
            if sum > 0 {
                let average = sum / count as u64;
                passer.systems.gfx.set_text(
                    &mut passer.systems.renderer,
                    &passer.content.game_content.interface.average_ping,
                    &format!("Av. Ping: {average:?}"),
                );
            }
            if count >= 20 {
                passer
                    .content
                    .game_content
                    .interface
                    .average_ping_collection
                    .pop_back();
            }
        }
        passer
            .content
            .game_content
            .interface
            .average_ping_collection
            .push_front(elapse_time);
    }

    passer.systems.gfx.set_text(
        &mut passer.systems.renderer,
        &passer.content.game_content.interface.ping_text,
        &format!("Ping: {elapse_time:?}"),
    );

    Ok(())
}

pub fn handle_handshake(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let code = data.read::<String>()?;
    let handshake = data.read::<String>()?;
    passer.systems.config.reconnect_code = code;
    passer.systems.config.save_config("settings.toml");
    passer.content.game_content.reconnect_count = 0;
    send_handshake(passer.socket, handshake)
}

pub fn handle_loginok(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let _hour = data.read::<u32>()?;
    let _min = data.read::<u32>()?;

    passer.systems.fade.init_fade(
        &mut passer.systems.gfx,
        FadeType::In,
        FADE_SWITCH_TO_GAME,
        FadeData::None,
    );

    send_login_ok(passer.socket, &passer.systems.config.reconnect_code)
}

pub fn handle_myindex(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let entity = data.read::<GlobalKey>()?;
    passer.content.game_content.myentity = Some(entity);
    Ok(())
}

pub fn handle_tls_handshake(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let code = data.read::<String>()?;
    let handshake = data.read::<String>()?;
    passer.systems.config.reconnect_code = code;
    passer.systems.config.save_config("settings.toml");
    passer.content.game_content.reconnect_count = 0;
    send_tls_handshake(passer.socket, handshake)
}

pub fn handle_clear_data(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let _ = data.read::<u32>()?;

    passer
        .systems
        .gfx
        .set_color(&passer.systems.fade.f_image, Color::rgba(0, 0, 0, 255));
    passer.systems.fade.f_alpha = 255;

    passer.content.game_content.clear_data(
        passer.world,
        passer.systems,
        &mut passer.graphics.map_renderer,
    )?;
    passer.content.game_content.show(passer.systems);
    Ok(())
}

pub fn handle_playitemsfx(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let index = data.read::<u16>()?;

    if let Some(sfx_name) =
        &passer.systems.base.item[index as usize].sound_index
    {
        let volume = passer.systems.config.sfx_volume as f32 * 0.01;
        passer
            .systems
            .audio
            .play_effect(format!("./audio/{sfx_name}"), volume)?;
    }

    Ok(())
}
