use crate::{socket::{error, *}, Position, values::*};
use bytey::ByteBuffer;

#[repr(u32)]
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
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Register as u32)?;
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
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Login as u32)?;
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
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Move as u32)?;
    buf.write(dir)?;
    buf.write(pos)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_dir(
    socket: &mut Socket,
    dir: u8,
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Dir as u32)?;
    buf.write(dir)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_attack(
    socket: &mut Socket,
    dir: u8,
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Attack as u32)?;
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
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::UseItem as u32)?;
    buf.write(slot)?;
    buf.write(targettype)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_unequip(
    socket: &mut Socket,
    slot: u16,
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Unequip as u32)?;
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
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::SwitchInvSlot as u32)?;
    buf.write(oldslot)?;
    buf.write(newslot)?;
    buf.write(amount)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_pickup(
    socket: &mut Socket,
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::PickUp as u32)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_dropitem(
    socket: &mut Socket,
    slot: u16,
    amount: u16,
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::DropItem as u32)?;
    buf.write(slot)?;
    buf.write(amount)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_deleteitem(
    socket: &mut Socket,
    slot: u16,
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::DeleteItem as u32)?;
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
) -> Result<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Message as u32)?;
    buf.write(channel)?;
    buf.write(msg)?;
    buf.write(name)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}