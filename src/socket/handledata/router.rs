use crate::{fade::*, socket::*, BufferTask, Entity, Position};

#[allow(clippy::too_many_arguments)]
pub fn handle_data(
    socket: &mut Socket,
    router: &PacketRouter,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    alert: &mut Alert,
    data: &mut ByteBuffer,
    seconds: f32,
    buffer: &mut BufferTask,
) -> SocketResult<()> {
    let id: ServerPackets = data.read()?;

    //println!("Receiving Packet ID {:?}", id);

    let fun = match router.0.get(&id) {
        Some(fun) => fun,
        None => return Err(AscendingSocketError::InvalidPacket),
    };

    match fun(
        socket, world, systems, content, alert, data, seconds, buffer,
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error {}", e);
            Err(e)
        }
    }
}
