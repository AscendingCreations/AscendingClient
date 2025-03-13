use mmap_bytey::MByteBuffer;

use crate::{
    Alert, AlertIndex, AlertType, Entity, GlobalKey, IsUsingType, Item, Result,
    TradeStatus, World,
    content::{Content, Window, open_interface},
    systems::{BufferTask, Poller, SystemHolder},
};

pub fn handle_updatetradeitem(
    _socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let same_entity = data.read::<bool>()?;
    let trade_slot = data.read::<u16>()?;
    let item = data.read::<Item>()?;

    content.game_content.interface.trade.update_trade_slot(
        systems,
        trade_slot as usize,
        &item,
        same_entity,
    );

    Ok(())
}

pub fn handle_updatetrademoney(
    _socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let amount = data.read::<u64>()?;

    content
        .game_content
        .interface
        .trade
        .update_trade_money(systems, amount);

    Ok(())
}

pub fn handle_inittrade(
    _socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let target_entity = data.read::<GlobalKey>()?;

    content.game_content.player_data.is_using_type =
        IsUsingType::Trading(target_entity);

    open_interface(&mut content.game_content.interface, systems, Window::Trade);
    content
        .game_content
        .interface
        .trade
        .clear_trade_items(systems);

    content
        .game_content
        .keyinput
        .iter_mut()
        .for_each(|key_press| *key_press = false);

    Ok(())
}

pub fn handle_tradestatus(
    _socket: &mut Poller,
    _world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    _alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let my_status = data.read::<TradeStatus>()?;
    let their_status = data.read::<TradeStatus>()?;

    content.game_content.interface.trade.trade_status = my_status;

    if my_status == TradeStatus::Accepted
        && their_status == TradeStatus::Accepted
    {
        content.game_content.interface.trade.button[1]
            .change_text(systems, "Confirm".into());
        content.game_content.interface.trade.update_status(
            systems,
            "Click the 'Confirm' Button to proceed".into(),
        );
    }

    match my_status {
        TradeStatus::None => {
            content
                .game_content
                .interface
                .trade
                .update_my_status(systems, "My Trade: Preparing...".into());
        }
        TradeStatus::Accepted => {
            content
                .game_content
                .interface
                .trade
                .update_my_status(systems, "My Trade: Submitted".into());
        }
        TradeStatus::Submitted => {
            content
                .game_content
                .interface
                .trade
                .update_my_status(systems, "My Trade: Confirmed".into());
        }
    }
    match their_status {
        TradeStatus::None => {
            content.game_content.interface.trade.update_their_status(
                systems,
                "Their Trade: Preparing...".into(),
            );
        }
        TradeStatus::Accepted => {
            content
                .game_content
                .interface
                .trade
                .update_their_status(systems, "Their Trade: Submitted".into());
        }
        TradeStatus::Submitted => {
            content
                .game_content
                .interface
                .trade
                .update_their_status(systems, "Their Trade: Confirmed".into());
        }
    }

    Ok(())
}

pub fn handle_traderequest(
    _socket: &mut Poller,
    world: &mut World,
    systems: &mut SystemHolder,
    _content: &mut Content,
    alert: &mut Alert,
    data: &mut MByteBuffer,
    _seconds: f32,
    _buffer: &mut BufferTask,
) -> Result<()> {
    let entity = data.read::<GlobalKey>()?;
    if !world.entities.contains_key(entity) {
        return Ok(());
    }

    let name = if let Some(Entity::Player(p_data)) = world.entities.get(entity)
    {
        p_data.entity_name.0.clone()
    } else {
        return Ok(());
    };

    alert.show_alert(
        systems,
        AlertType::Confirm,
        "Would you like to accept this trade request?".into(),
        format!("{name} would like to trade with you"),
        250,
        AlertIndex::TradeRequest,
        false,
    );

    Ok(())
}
