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
) -> ClientResult<()> {
    let id: ServerPackets = data.read()?;

    let fun = match router.0.get(&id) {
        Some(fun) => fun,
        None => return Err(ClientError::InvalidPacket),
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
