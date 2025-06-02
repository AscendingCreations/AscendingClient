use mmap_bytey::MByteBuffer;

use crate::{
    Alert, AlertIndex, AlertType, COLOR_BLUE, COLOR_GREEN, COLOR_RED,
    COLOR_WHITE, FtlType, GlobalKey, IsUsingType, MessageChannel, Position,
    Result, UserAccess, World,
    content::{Content, Window, add_float_text, open_interface},
    systems::{BufferTask, ChatTask, Poller, SystemHolder},
};

pub fn handle_alertmsg(
    _socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let message = data.read::<String>()?;
    let _close = data.read::<u8>()?;

    alert.show_alert(
        systems,
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
    _socket: &mut Poller,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _flttype = data.read::<FtlType>()?;
    let _message = data.read::<String>()?;

    Ok(())
}

pub fn handle_chatmsg(
    _socket: &mut Poller,
    _world: &mut World,
    _systems: &mut SystemHolder,
    _content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    buffer: &mut BufferTask,
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

        buffer.chatbuffer.add_task(ChatTask::new(
            (msg_string, COLOR_WHITE),
            header,
            channel,
        ));
    }

    Ok(())
}

pub fn handle_openstorage(
    _socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let _ = data.read::<u32>()?;

    open_interface(
        &mut content.game_content.interface,
        systems,
        Window::Storage,
    );

    content
        .game_content
        .keyinput
        .iter_mut()
        .for_each(|key_press| *key_press = false);

    content.game_content.player_data.is_using_type = IsUsingType::Bank;

    Ok(())
}

pub fn handle_openshop(
    _socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let shop_index = data.read::<u16>()?;

    open_interface(&mut content.game_content.interface, systems, Window::Shop);
    content
        .game_content
        .interface
        .shop
        .set_shop(systems, shop_index as usize);

    content
        .game_content
        .keyinput
        .iter_mut()
        .for_each(|key_press| *key_press = false);

    content.game_content.player_data.is_using_type =
        IsUsingType::Store(shop_index as i64);

    Ok(())
}

pub fn handle_damage(
    _socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
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

        add_float_text(systems, &mut content.game_content, pos, text, color);
    }

    Ok(())
}
