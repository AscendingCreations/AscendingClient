use crate::{socket::{error, *}, Position, values::*};
use bytey::ByteBuffer;

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    ByteBufferRead,
    ByteBufferWrite,
)]
enum ClientPacket {
    Register,
    Login,
    Move,
    Dir,
    Attack,
    UseItem,
    Unequip,
    SwitchInvSlot,
    PickUp,
    DropItem,
    DeleteItem,
    Message,
}

pub fn send_register(
    socket: &mut Socket,
    username: String,
    password: String,
    email: String,
    sprite: u8,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Register)?;
    buf.write(username)?;
    buf.write(password)?;
    buf.write(email)?;
    buf.write(sprite)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_login(
    socket: &mut Socket,
    username: String,
    password: String,
    app_version: (u16, u16, u16),
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(778)?;

    buf.write(ClientPacket::Login)?;
    buf.write(username)?;
    buf.write(password)?;
    buf.write(app_version.0)?;
    buf.write(app_version.1)?;
    buf.write(app_version.2)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_move(
    socket: &mut Socket,
    dir: u8,
    pos: Position,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(25)?;

    buf.write(ClientPacket::Move)?;
    buf.write(dir)?;
    buf.write(pos)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_dir(
    socket: &mut Socket,
    dir: u8,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Dir)?;
    buf.write(dir)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_attack(
    socket: &mut Socket,
    dir: u8,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Attack)?;
    buf.write(dir)?;
    buf.write(1 as u8)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_useitem(
    socket: &mut Socket,
    slot: u16,
    targettype: u8,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::UseItem)?;
    buf.write(slot)?;
    buf.write(targettype)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_unequip(
    socket: &mut Socket,
    slot: u16,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Unequip)?;
    buf.write(slot)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_switchinvslot(
    socket: &mut Socket,
    oldslot: u16,
    newslot: u16,
    amount: u16,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::SwitchInvSlot)?;
    buf.write(oldslot)?;
    buf.write(newslot)?;
    buf.write(amount)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_pickup(
    socket: &mut Socket,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::PickUp)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_dropitem(
    socket: &mut Socket,
    slot: u16,
    amount: u16,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::DropItem)?;
    buf.write(slot)?;
    buf.write(amount)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_deleteitem(
    socket: &mut Socket,
    slot: u16,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::DeleteItem)?;
    buf.write(slot)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_message(
    socket: &mut Socket,
    channel: MessageChannel,
    msg: String,
    name: String,
) -> SocketResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Message)?;
    buf.write(channel)?;
    buf.write(msg)?;
    buf.write(name)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}