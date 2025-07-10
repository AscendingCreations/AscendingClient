use mmap_bytey::MByteBuffer;

use crate::{
    Alert, AlertIndex, AlertType, COLOR_BLUE, COLOR_GREEN, COLOR_RED,
    COLOR_WHITE, FtlType, GlobalKey, IsUsingType, MessageChannel, Position,
    Result, UserAccess, World,
    content::{Content, Window, add_float_text, open_interface},
    systems::{
        BufferTask, ChatTask, Poller, SystemHolder, mapper::PacketPasser,
    },
};

pub fn handle_alertmsg(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let message = data.read::<String>()?;
    let _close = data.read::<u8>()?;

    passer.alert.show_alert(
        passer.systems,
        AlertType::Inform,
        message,
        "Alert Message".into(),
        250,
        AlertIndex::None,
        false,
    );

    Ok(())
}

pub fn handle_fltalert(
    data: &mut MByteBuffer,
    _passer: &mut PacketPasser,
) -> Result<()> {
    let _flttype = data.read::<FtlType>()?;
    let _message = data.read::<String>()?;

    Ok(())
}

pub fn handle_chatmsg(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let channel = data.read::<MessageChannel>()?;
        let head_string = data.read::<String>()?;
        let msg_string = data.read::<String>()?;
        let _useraccess = data.read::<Option<UserAccess>>()?;

        let header = if !head_string.is_empty() {
            let color = match channel {
                MessageChannel::Global => COLOR_GREEN,
                MessageChannel::Map => COLOR_BLUE,
                MessageChannel::Private => COLOR_RED,
                _ => COLOR_WHITE,
            };
            Some((head_string, color))
        } else {
            None
        };

        passer.buffer.chatbuffer.add_task(ChatTask::new(
            (msg_string, COLOR_WHITE),
            header,
            channel,
        ));
    }

    Ok(())
}

pub fn handle_openstorage(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let _ = data.read::<u32>()?;

    open_interface(
        &mut passer.content.game_content.interface,
        passer.systems,
        Window::Storage,
    );

    passer
        .content
        .game_content
        .keyinput
        .iter_mut()
        .for_each(|key_press| *key_press = false);

    passer.content.game_content.player_data.is_using_type = IsUsingType::Bank;

    Ok(())
}

pub fn handle_openshop(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let shop_index = data.read::<u16>()?;

    open_interface(
        &mut passer.content.game_content.interface,
        passer.systems,
        Window::Shop,
    );
    passer
        .content
        .game_content
        .interface
        .shop
        .set_shop(passer.systems, shop_index as usize);

    passer
        .content
        .game_content
        .keyinput
        .iter_mut()
        .for_each(|key_press| *key_press = false);

    passer.content.game_content.player_data.is_using_type =
        IsUsingType::Store(shop_index as i64);

    Ok(())
}

pub fn handle_damage(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let count = data.read::<u32>()?;

    for _ in 0..count {
        let _entity = data.read::<GlobalKey>()?;
        let amount = data.read::<u16>()?;
        let pos = data.read::<Position>()?;
        let is_damage = data.read::<bool>()?;

        let (text, color) = if is_damage {
            (format!("-{amount}"), COLOR_RED)
        } else {
            (format!("+{amount}"), COLOR_GREEN)
        };

        add_float_text(
            passer.systems,
            &mut passer.content.game_content,
            pos,
            text,
            color,
        );
    }

    Ok(())
}
