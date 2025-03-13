use crate::{BufferTask, data_types::*, fade::*, socket::*};
use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use super::{
    handle_entity::*, handle_general::*, handle_interface::*, handle_player::*,
    handle_trade::*,
};

type PacketFunction = fn(
    &mut Poller,
    &mut World,
    &mut SystemHolder,
    &mut Content,
    &mut Alert,
    &mut MByteBuffer,
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
    MByteBufferRead,
    MByteBufferWrite,
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
    TlsHandShake,
    ClearData,
}

pub struct PacketRouter(pub AHashMap<ServerPackets, PacketFunction>);

impl PacketRouter {
    pub fn init() -> Self {
        Self(AHashMap::from([
            (ServerPackets::AlertMsg, handle_alertmsg as PacketFunction),
            (ServerPackets::FltAlert, handle_fltalert as PacketFunction),
            (ServerPackets::LoginOk, handle_loginok as PacketFunction),
            (ServerPackets::MapItems, handle_mapitems as PacketFunction),
            (ServerPackets::MyIndex, handle_myindex as PacketFunction),
            (
                ServerPackets::PlayerData,
                handle_playerdata as PacketFunction,
            ),
            (
                ServerPackets::PlayerSpawn,
                handle_playerspawn as PacketFunction,
            ),
            (ServerPackets::Move, handle_move as PacketFunction),
            (ServerPackets::MoveOk, handle_move_ok as PacketFunction),
            (ServerPackets::Warp, handle_warp as PacketFunction),
            (ServerPackets::Dir, handle_dir as PacketFunction),
            (ServerPackets::Vitals, handle_vitals as PacketFunction),
            (ServerPackets::PlayerInv, handle_playerinv as PacketFunction),
            (
                ServerPackets::PlayerInvSlot,
                handle_playerinvslot as PacketFunction,
            ),
            (
                ServerPackets::PlayerStorage,
                handle_playerstorage as PacketFunction,
            ),
            (
                ServerPackets::PlayerStorageSlot,
                handle_playerstorageslot as PacketFunction,
            ),
            (ServerPackets::Attack, handle_attack as PacketFunction),
            (
                ServerPackets::PlayerEquipment,
                handle_playerequipment as PacketFunction,
            ),
            (
                ServerPackets::PlayerLevel,
                handle_playerlevel as PacketFunction,
            ),
            (
                ServerPackets::PlayerMoney,
                handle_playermoney as PacketFunction,
            ),
            (ServerPackets::Death, handle_death as PacketFunction),
            (ServerPackets::PlayerPk, handle_playerpk as PacketFunction),
            (ServerPackets::NpcData, handle_npcdata as PacketFunction),
            (ServerPackets::ChatMsg, handle_chatmsg as PacketFunction),
            (
                ServerPackets::EntityUnload,
                handle_entityunload as PacketFunction,
            ),
            (
                ServerPackets::OpenStorage,
                handle_openstorage as PacketFunction,
            ),
            (ServerPackets::OpenShop, handle_openshop as PacketFunction),
            (
                ServerPackets::ClearIsUsingType,
                handle_clearisusingtype as PacketFunction,
            ),
            (
                ServerPackets::UpdateTradeItem,
                handle_updatetradeitem as PacketFunction,
            ),
            (
                ServerPackets::UpdateTradeMoney,
                handle_updatetrademoney as PacketFunction,
            ),
            (ServerPackets::InitTrade, handle_inittrade as PacketFunction),
            (ServerPackets::HandShake, handle_handshake as PacketFunction),
            (
                ServerPackets::TradeStatus,
                handle_tradestatus as PacketFunction,
            ),
            (
                ServerPackets::TradeRequest,
                handle_traderequest as PacketFunction,
            ),
            (
                ServerPackets::PlayItemSfx,
                handle_playitemsfx as PacketFunction,
            ),
            (ServerPackets::Damage, handle_damage as PacketFunction),
            (ServerPackets::Ping, handle_ping as PacketFunction),
            (
                ServerPackets::TlsHandShake,
                handle_tls_handshake as PacketFunction,
            ),
            (
                ServerPackets::ClearData,
                handle_clear_data as PacketFunction,
            ),
        ]))
    }
}
