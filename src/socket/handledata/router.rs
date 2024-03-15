use crate::{socket::*, Entity, Position, fade::*};

pub fn handle_data(socket: &mut Socket, router: &PacketRouter, world: &mut World, systems: &mut DrawSetting, content: &mut Content, alert: &mut Alert, data: &mut ByteBuffer) -> SocketResult<()> {
    let id: ServerPackets = data.read()?;

    println!("Receiving Packet ID {:?}", id);

    let fun = match router.0.get(&id) {
        Some(fun) => fun,
        None => return Err(AscendingSocketError::InvalidPacket),
    };

    match fun(socket, world, systems, content, alert, data) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error {}", e);
            Err(e)
        }
    }
}