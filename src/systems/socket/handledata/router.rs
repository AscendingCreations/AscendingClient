use log::{error, trace};
use snafu::Backtrace;

use crate::{BufferTask, data_types::*, fade::*, socket::*};

#[allow(clippy::too_many_arguments)]
pub fn handle_data(
    socket: &mut Poller,
    router: &PacketRouter,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    alert: &mut Alert,
    data: &mut MByteBuffer,
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
        None => {
            return Err(ClientError::InvalidPacket {
                backtrace: Backtrace::new(),
            });
        }
    };

    match fun(
        socket, world, systems, content, alert, data, seconds, buffer,
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error {:?} {}", id, e);
            Err(e)
        }
    }
}
