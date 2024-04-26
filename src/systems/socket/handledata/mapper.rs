use crate::{data_types::*, fade::*, socket::*, BufferTask};

use bytey::{ByteBuffer, ByteBufferRead, ByteBufferWrite};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type PacketFunction = fn(
    &mut Socket,
    &mut World,
    &mut SystemHolder,
    &mut Content,
    &mut Alert,
    &mut ByteBuffer,
    f32,
    &mut BufferTask,
) -> Result<()>;

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    ByteBufferRead,
    ByteBufferWrite,
    Hash,
)]
pub enum ServerPackets {
    OnlineCheck,
    AlertMsg,
    FltAlert,
    HandShake,
    LoginOk,
    MapItems,
    MyIndex,
    Move,
    MoveOk,
    Warp,
    Dir,
    Vitals,
    Attack,
    Death,
    PlayerData,
    PlayerSpawn,
    PlayerInv,
    PlayerInvSlot,
    PlayerStorage,
    PlayerStorageSlot,
    PlayerEquipment,
    PlayerLevel,
    PlayerMoney,
    PlayerPk,
    NpcData,
    ChatMsg,
    EntityUnload,
    OpenStorage,
    OpenShop,
    ClearIsUsingType,
    UpdateTradeItem,
    UpdateTradeMoney,
    InitTrade,
    TradeStatus,
    TradeRequest,
    PlayItemSfx,
    Damage,
    Ping,
}

pub struct PacketRouter(pub HashMap<ServerPackets, PacketFunction>);

impl PacketRouter {
    pub fn init() -> Self {
        Self(HashMap::from([
            (
                ServerPackets::AlertMsg,
                routes::handle_alertmsg as PacketFunction,
            ),
            (
                ServerPackets::FltAlert,
                routes::handle_fltalert as PacketFunction,
            ),
            (
                ServerPackets::LoginOk,
                routes::handle_loginok as PacketFunction,
            ),
            (
                ServerPackets::MapItems,
                routes::handle_mapitems as PacketFunction,
            ),
            (
                ServerPackets::MyIndex,
                routes::handle_myindex as PacketFunction,
            ),
            (
                ServerPackets::PlayerData,
                routes::handle_playerdata as PacketFunction,
            ),
            (
                ServerPackets::PlayerSpawn,
                routes::handle_playerspawn as PacketFunction,
            ),
            (ServerPackets::Move, routes::handle_move as PacketFunction),
            (
                ServerPackets::MoveOk,
                routes::handle_move_ok as PacketFunction,
            ),
            (ServerPackets::Warp, routes::handle_warp as PacketFunction),
            (ServerPackets::Dir, routes::handle_dir as PacketFunction),
            (
                ServerPackets::Vitals,
                routes::handle_vitals as PacketFunction,
            ),
            (
                ServerPackets::PlayerInv,
                routes::handle_playerinv as PacketFunction,
            ),
            (
                ServerPackets::PlayerInvSlot,
                routes::handle_playerinvslot as PacketFunction,
            ),
            (
                ServerPackets::PlayerStorage,
                routes::handle_playerstorage as PacketFunction,
            ),
            (
                ServerPackets::PlayerStorageSlot,
                routes::handle_playerstorageslot as PacketFunction,
            ),
            (
                ServerPackets::Attack,
                routes::handle_attack as PacketFunction,
            ),
            (
                ServerPackets::PlayerEquipment,
                routes::handle_playerequipment as PacketFunction,
            ),
            (
                ServerPackets::PlayerLevel,
                routes::handle_playerlevel as PacketFunction,
            ),
            (
                ServerPackets::PlayerMoney,
                routes::handle_playermoney as PacketFunction,
            ),
            (ServerPackets::Death, routes::handle_death as PacketFunction),
            (
                ServerPackets::PlayerPk,
                routes::handle_playerpk as PacketFunction,
            ),
            (
                ServerPackets::NpcData,
                routes::handle_npcdata as PacketFunction,
            ),
            (
                ServerPackets::ChatMsg,
                routes::handle_chatmsg as PacketFunction,
            ),
            (
                ServerPackets::EntityUnload,
                routes::handle_entityunload as PacketFunction,
            ),
            (
                ServerPackets::OpenStorage,
                routes::handle_openstorage as PacketFunction,
            ),
            (
                ServerPackets::OpenShop,
                routes::handle_openshop as PacketFunction,
            ),
            (
                ServerPackets::ClearIsUsingType,
                routes::handle_clearisusingtype as PacketFunction,
            ),
            (
                ServerPackets::UpdateTradeItem,
                routes::handle_updatetradeitem as PacketFunction,
            ),
            (
                ServerPackets::UpdateTradeMoney,
                routes::handle_updatetrademoney as PacketFunction,
            ),
            (
                ServerPackets::InitTrade,
                routes::handle_inittrade as PacketFunction,
            ),
            (
                ServerPackets::HandShake,
                routes::handle_handshake as PacketFunction,
            ),
            (
                ServerPackets::TradeStatus,
                routes::handle_tradestatus as PacketFunction,
            ),
            (
                ServerPackets::TradeRequest,
                routes::handle_traderequest as PacketFunction,
            ),
            (
                ServerPackets::PlayItemSfx,
                routes::handle_playitemsfx as PacketFunction,
            ),
            (
                ServerPackets::Damage,
                routes::handle_damage as PacketFunction,
            ),
            (ServerPackets::Ping, routes::handle_ping as PacketFunction),
        ]))
    }
}
