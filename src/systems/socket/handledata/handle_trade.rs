use mmap_bytey::MByteBuffer;

use crate::{
    Alert, AlertIndex, AlertType, Entity, GlobalKey, IsUsingType, Item, Result,
    TradeStatus, World,
    content::{Content, Window, open_interface},
    systems::{BufferTask, Poller, SystemHolder, mapper::PacketPasser},
};

pub fn handle_updatetradeitem(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let same_entity = data.read::<bool>()?;
    let trade_slot = data.read::<u16>()?;
    let item = data.read::<Item>()?;

    passer
        .content
        .game_content
        .interface
        .trade
        .update_trade_slot(
            passer.systems,
            trade_slot as usize,
            &item,
            same_entity,
        );

    Ok(())
}

pub fn handle_updatetrademoney(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let amount = data.read::<u64>()?;

    passer
        .content
        .game_content
        .interface
        .trade
        .update_trade_money(passer.systems, amount);

    Ok(())
}

pub fn handle_inittrade(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let target_entity = data.read::<GlobalKey>()?;

    passer.content.game_content.player_data.is_using_type =
        IsUsingType::Trading(target_entity);

    open_interface(
        &mut passer.content.game_content.interface,
        passer.systems,
        Window::Trade,
    );
    passer
        .content
        .game_content
        .interface
        .trade
        .clear_trade_items(passer.systems);

    passer
        .content
        .game_content
        .keyinput
        .iter_mut()
        .for_each(|key_press| *key_press = false);

    Ok(())
}

pub fn handle_tradestatus(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let my_status = data.read::<TradeStatus>()?;
    let their_status = data.read::<TradeStatus>()?;

    passer.content.game_content.interface.trade.trade_status = my_status;

    if my_status == TradeStatus::Accepted
        && their_status == TradeStatus::Accepted
    {
        passer.content.game_content.interface.trade.button[1]
            .change_text(passer.systems, "Confirm".into());
        passer.content.game_content.interface.trade.update_status(
            passer.systems,
            "Click the 'Confirm' Button to proceed".into(),
        );
    }

    match my_status {
        TradeStatus::None => {
            passer
                .content
                .game_content
                .interface
                .trade
                .update_my_status(
                    passer.systems,
                    "My Trade: Preparing...".into(),
                );
        }
        TradeStatus::Accepted => {
            passer
                .content
                .game_content
                .interface
                .trade
                .update_my_status(passer.systems, "My Trade: Submitted".into());
        }
        TradeStatus::Submitted => {
            passer
                .content
                .game_content
                .interface
                .trade
                .update_my_status(passer.systems, "My Trade: Confirmed".into());
        }
    }
    match their_status {
        TradeStatus::None => {
            passer
                .content
                .game_content
                .interface
                .trade
                .update_their_status(
                    passer.systems,
                    "Their Trade: Preparing...".into(),
                );
        }
        TradeStatus::Accepted => {
            passer
                .content
                .game_content
                .interface
                .trade
                .update_their_status(
                    passer.systems,
                    "Their Trade: Submitted".into(),
                );
        }
        TradeStatus::Submitted => {
            passer
                .content
                .game_content
                .interface
                .trade
                .update_their_status(
                    passer.systems,
                    "Their Trade: Confirmed".into(),
                );
        }
    }

    Ok(())
}

pub fn handle_traderequest(
    data: &mut MByteBuffer,
    passer: &mut PacketPasser,
) -> Result<()> {
    let entity = data.read::<GlobalKey>()?;
    if !passer.world.entities.contains_key(entity) {
        return Ok(());
    }

    let name = if let Some(Entity::Player(p_data)) =
        passer.world.entities.get(entity)
    {
        p_data.entity_name.0.clone()
    } else {
        return Ok(());
    };

    passer.alert.show_alert(
        passer.systems,
        AlertType::Confirm,
        "Would you like to accept this trade request?".into(),
        format!("{name} would like to trade with you"),
        250,
        AlertIndex::TradeRequest,
        false,
    );

    Ok(())
}
