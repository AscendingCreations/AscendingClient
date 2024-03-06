use crate::socket::{*, error};
use bytey::ByteBuffer;

#[repr(u32)]
enum ClientPacket {
    Register,
}

pub fn send_register(socket: &mut Socket) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Register as u32)?;
    buf.write("genusis")?;
    buf.write("damit1")?;
    buf.write("lordsatin@hotmail.com")?;
    buf.write(1u8)?;
    buf.write(1u8)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}