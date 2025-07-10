use crate::{BufferTask, data_types::*, fade::*, socket::*};
use ahash::AHashMap;
use graphics::MapRenderer;
use serde::{Deserialize, Serialize};

use super::{
    handle_entity::*, handle_general::*, handle_interface::*, handle_player::*,
    handle_trade::*,
};

pub struct PacketPasser<'a> {
    pub socket: &'a mut Poller,
    pub world: &'a mut World,
    pub systems: &'a mut SystemHolder,
    pub content: &'a mut Content,
    pub alert: &'a mut Alert,
    pub map_renderer: &'a mut MapRenderer,
    pub seconds: f32,
    pub buffer: &'a mut BufferTask,
}

type PacketFunction = fn(&mut MByteBuffer, &mut PacketPasser) -> Result<()>;

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

pub fn run_packet(packet: &ServerPackets) -> Option<PacketFunction> {
    match packet {
        ServerPackets::AlertMsg => Some(handle_alertmsg as PacketFunction),
        ServerPackets::FltAlert => Some(handle_fltalert as PacketFunction),
        ServerPackets::LoginOk => Some(handle_loginok as PacketFunction),
        ServerPackets::MapItems => Some(handle_mapitems as PacketFunction),
        ServerPackets::MyIndex => Some(handle_myindex as PacketFunction),
        ServerPackets::PlayerData => Some(handle_playerdata as PacketFunction),
        ServerPackets::PlayerSpawn => {
            Some(handle_playerspawn as PacketFunction)
        }
        ServerPackets::Move => Some(handle_move as PacketFunction),
        ServerPackets::MoveOk => Some(handle_move_ok as PacketFunction),
        ServerPackets::Warp => Some(handle_warp as PacketFunction),
        ServerPackets::Dir => Some(handle_dir as PacketFunction),
        ServerPackets::Vitals => Some(handle_vitals as PacketFunction),
        ServerPackets::PlayerInv => Some(handle_playerinv as PacketFunction),
        ServerPackets::PlayerInvSlot => {
            Some(handle_playerinvslot as PacketFunction)
        }
        ServerPackets::PlayerStorage => {
            Some(handle_playerstorage as PacketFunction)
        }
        ServerPackets::PlayerStorageSlot => {
            Some(handle_playerstorageslot as PacketFunction)
        }
        ServerPackets::Attack => Some(handle_attack as PacketFunction),
        ServerPackets::PlayerEquipment => {
            Some(handle_playerequipment as PacketFunction)
        }
        ServerPackets::PlayerLevel => {
            Some(handle_playerlevel as PacketFunction)
        }
        ServerPackets::PlayerMoney => {
            Some(handle_playermoney as PacketFunction)
        }
        ServerPackets::Death => Some(handle_death as PacketFunction),
        ServerPackets::PlayerPk => Some(handle_playerpk as PacketFunction),
        ServerPackets::NpcData => Some(handle_npcdata as PacketFunction),
        ServerPackets::ChatMsg => Some(handle_chatmsg as PacketFunction),
        ServerPackets::EntityUnload => {
            Some(handle_entityunload as PacketFunction)
        }
        ServerPackets::OpenStorage => {
            Some(handle_openstorage as PacketFunction)
        }
        ServerPackets::OpenShop => Some(handle_openshop as PacketFunction),
        ServerPackets::ClearIsUsingType => {
            Some(handle_clearisusingtype as PacketFunction)
        }
        ServerPackets::UpdateTradeItem => {
            Some(handle_updatetradeitem as PacketFunction)
        }
        ServerPackets::UpdateTradeMoney => {
            Some(handle_updatetrademoney as PacketFunction)
        }
        ServerPackets::InitTrade => Some(handle_inittrade as PacketFunction),
        ServerPackets::HandShake => Some(handle_handshake as PacketFunction),
        ServerPackets::TradeStatus => {
            Some(handle_tradestatus as PacketFunction)
        }
        ServerPackets::TradeRequest => {
            Some(handle_traderequest as PacketFunction)
        }
        ServerPackets::PlayItemSfx => {
            Some(handle_playitemsfx as PacketFunction)
        }
        ServerPackets::Damage => Some(handle_damage as PacketFunction),
        ServerPackets::Ping => Some(handle_ping as PacketFunction),
        ServerPackets::TlsHandShake => {
            Some(handle_tls_handshake as PacketFunction)
        }
        ServerPackets::ClearData => Some(handle_clear_data as PacketFunction),
        ServerPackets::OnlineCheck => None,
    }
}
