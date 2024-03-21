use crate::{fade::*, socket::*, BufferTask, Entity, Position};

use bytey::{ByteBuffer, ByteBufferRead, ByteBufferWrite};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type PacketFunction = fn(
    &mut Socket,
    &mut World,
    &mut DrawSetting,
    &mut Content,
    &mut Alert,
    &mut ByteBuffer,
    f32,
    &mut BufferTask,
) -> SocketResult<()>;

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
    Ping,
    Status,
    AlertMsg,
    FltAlert,
    LoginOk,
    Ingame,
    UpdateMap,
    MapItems,
    MyIndex,
    PlayerData,
    PlayerSpawn,
    PlayerMove,
    PlayerWarp,
    PlayerMapSwap,
    Dataremovelist,
    Dataremove,
    PlayerDir,
    PlayerVitals,
    PlayerInv,
    PlayerInvSlot,
    KeyInput,
    PlayerAttack,
    PlayerEquipment,
    PlayerAction,
    PlayerLevel,
    PlayerMoney,
    PlayerStun,
    PlayerVariables,
    PlayerVariable,
    PlayerDeath,
    NpcDeath,
    PlayerPvp,
    PlayerPk,
    PlayerEmail,
    NpcData,
    NpcMove,
    NpcWarp,
    NpcDir,
    NpcVital,
    NpcAttack,
    NpcStun,
    ChatMsg,
    Sound,
    Target,
    SyncCheck,
    EntityUnload,
    LoadStatus,
    ServerPacketCount,
}

pub struct PacketRouter(pub HashMap<ServerPackets, PacketFunction>);

impl PacketRouter {
    pub fn init() -> Self {
        Self(HashMap::from([
            (ServerPackets::Ping, routes::handle_ping as PacketFunction),
            (
                ServerPackets::Status,
                routes::handle_status as PacketFunction,
            ),
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
                ServerPackets::Ingame,
                routes::handle_ingame as PacketFunction,
            ),
            (
                ServerPackets::UpdateMap,
                routes::handle_updatemap as PacketFunction,
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
            (
                ServerPackets::PlayerMove,
                routes::handle_playermove as PacketFunction,
            ),
            (
                ServerPackets::PlayerWarp,
                routes::handle_playerwarp as PacketFunction,
            ),
            (
                ServerPackets::PlayerMapSwap,
                routes::handle_playermapswap as PacketFunction,
            ),
            (
                ServerPackets::Dataremovelist,
                routes::handle_dataremovelist as PacketFunction,
            ),
            (
                ServerPackets::Dataremove,
                routes::handle_dataremove as PacketFunction,
            ),
            (
                ServerPackets::PlayerDir,
                routes::handle_playerdir as PacketFunction,
            ),
            (
                ServerPackets::PlayerVitals,
                routes::handle_playervitals as PacketFunction,
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
                ServerPackets::KeyInput,
                routes::handle_keyinput as PacketFunction,
            ),
            (
                ServerPackets::PlayerAttack,
                routes::handle_playerattack as PacketFunction,
            ),
            (
                ServerPackets::PlayerEquipment,
                routes::handle_playerequipment as PacketFunction,
            ),
            (
                ServerPackets::PlayerAction,
                routes::handle_playeraction as PacketFunction,
            ),
            (
                ServerPackets::PlayerLevel,
                routes::handle_playerlevel as PacketFunction,
            ),
            (
                ServerPackets::PlayerMoney,
                routes::handle_playermoney as PacketFunction,
            ),
            (
                ServerPackets::PlayerStun,
                routes::handle_playerstun as PacketFunction,
            ),
            (
                ServerPackets::PlayerVariables,
                routes::handle_playervariables as PacketFunction,
            ),
            (
                ServerPackets::PlayerVariable,
                routes::handle_playervariable as PacketFunction,
            ),
            (
                ServerPackets::PlayerDeath,
                routes::handle_playerdeath as PacketFunction,
            ),
            (
                ServerPackets::NpcDeath,
                routes::handle_npcdeath as PacketFunction,
            ),
            (
                ServerPackets::PlayerPvp,
                routes::handle_playerpvp as PacketFunction,
            ),
            (
                ServerPackets::PlayerPk,
                routes::handle_playerpk as PacketFunction,
            ),
            (
                ServerPackets::PlayerEmail,
                routes::handle_playeremail as PacketFunction,
            ),
            (
                ServerPackets::NpcData,
                routes::handle_npcdata as PacketFunction,
            ),
            (
                ServerPackets::NpcMove,
                routes::handle_npcmove as PacketFunction,
            ),
            (
                ServerPackets::NpcWarp,
                routes::handle_npcwarp as PacketFunction,
            ),
            (
                ServerPackets::NpcDir,
                routes::handle_npcdir as PacketFunction,
            ),
            (
                ServerPackets::NpcVital,
                routes::handle_npcvital as PacketFunction,
            ),
            (
                ServerPackets::NpcAttack,
                routes::handle_npcattack as PacketFunction,
            ),
            (
                ServerPackets::NpcStun,
                routes::handle_npcstun as PacketFunction,
            ),
            (
                ServerPackets::ChatMsg,
                routes::handle_chatmsg as PacketFunction,
            ),
            (ServerPackets::Sound, routes::handle_sound as PacketFunction),
            (
                ServerPackets::Target,
                routes::handle_target as PacketFunction,
            ),
            (
                ServerPackets::SyncCheck,
                routes::handle_synccheck as PacketFunction,
            ),
            (
                ServerPackets::EntityUnload,
                routes::handle_entityunload as PacketFunction,
            ),
            (
                ServerPackets::LoadStatus,
                routes::handle_loadstatus as PacketFunction,
            ),
        ]))
    }
}
