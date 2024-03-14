use crate::{socket::*, Entity, Position, fade::*};

pub fn handle_data(socket: &mut Socket, router: &PacketRouter, world: &mut World, systems: &mut DrawSetting, content: &mut Content, alert: &mut Alert, data: &mut ByteBuffer) -> SocketResult<()> {
    let id: ServerPackets = data.read()?;

    let fun = match router.0.get(&id) {
        Some(fun) => fun,
        None => return Err(AscendingSocketError::InvalidPacket),
    };

    fun(socket, world, systems, content, alert, data)
}