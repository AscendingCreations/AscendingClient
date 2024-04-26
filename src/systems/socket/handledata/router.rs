use log::{error, trace};

use crate::{data_types::*, fade::*, socket::*, BufferTask};

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
) -> Result<()> {
    let id: ServerPackets = data.read()?;

    trace!("Server Packet is {:?}", id);

    if id == ServerPackets::OnlineCheck {
        return Ok(());
    }

    let fun = match router.0.get(&id) {
        Some(fun) => fun,
        None => return Err(ClientError::InvalidPacket),
    };

    match fun(
        socket, world, systems, content, alert, data, seconds, buffer,
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error {}", e);
            Err(e)
        }
    }
}
