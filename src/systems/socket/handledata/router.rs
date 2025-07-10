use log::{error, trace};
use snafu::Backtrace;

use crate::{
    BufferTask, data_types::*, fade::*, socket::*,
    systems::mapper::PacketPasser,
};

#[allow(clippy::too_many_arguments)]
pub fn handle_data(
    socket: &mut Poller,
    router: &PacketRouter,
    world: &mut World,
    systems: &mut SystemHolder,
    map_renderer: &mut MapRenderer,
    content: &mut Content,
    alert: &mut Alert,
    data: &mut MByteBuffer,
    seconds: f32,
    buffer: &mut BufferTask,
) -> Result<()> {
    let id: ServerPackets = data.read()?;

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
        data,
        &mut PacketPasser {
            socket,
            world,
            systems,
            content,
            alert,
            map_renderer,
            seconds,
            buffer,
        },
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error {id:?} {e}");
            Err(e)
        }
    }
}
