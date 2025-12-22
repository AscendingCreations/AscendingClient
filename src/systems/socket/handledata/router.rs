use camera::controls::FlatControls;
use log::{error, trace};
use snafu::Backtrace;

use crate::{
    BufferTask,
    data_types::*,
    fade::*,
    socket::*,
    systems::{
        State,
        mapper::{PacketPasser, run_packet},
    },
};

#[allow(clippy::too_many_arguments)]
pub fn handle_data(
    socket: &mut Poller,
    world: &mut World,
    systems: &mut SystemHolder,
    content: &mut Content,
    alert: &mut Alert,
    data: &mut MByteBuffer,
    seconds: f32,
    buffer: &mut BufferTask,
    graphics: &mut State<FlatControls>,
) -> Result<()> {
    let id: ServerPackets = data.read()?;

    if id == ServerPackets::OnlineCheck {
        return Ok(());
    }

    let fun = match run_packet(&id) {
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
            seconds,
            buffer,
            graphics,
        },
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error {id:?} {e}");
            Err(e)
        }
    }
}
