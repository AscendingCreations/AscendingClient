use crate::{socket::*, values::*, Entity, Position};
use bytey::ByteBuffer;

#[derive(Clone, Debug, PartialEq, Eq, ByteBufferRead, ByteBufferWrite)]
pub enum AdminCommand {
    KickPlayer(String),
    WarpTo(Position),
    SpawnNpc(i32, Position),
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, ByteBufferRead, ByteBufferWrite,
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
    SwitchStorageSlot,
    DeleteStorageItem,
    DepositItem,
    WithdrawItem,
    Message,
    AdminCommand,
    SetTarget,
    CloseStorage,
    CloseShop,
    CloseTrade,
}

pub fn send_register(
    socket: &mut Socket,
    username: String,
    password: String,
    email: String,
    sprite: u8,
) -> ClientResult<()> {
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
) -> ClientResult<()> {
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
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(25)?;

    buf.write(ClientPacket::Move)?;
    buf.write(dir)?;
    buf.write(pos)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_dir(socket: &mut Socket, dir: u8) -> ClientResult<()> {
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
    entity: Option<Entity>,
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(13)?;

    buf.write(ClientPacket::Attack)?;
    buf.write(dir)?;
    buf.write(entity)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_useitem(
    socket: &mut Socket,
    slot: u16,
    targettype: u8,
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::UseItem)?;
    buf.write(slot)?;
    buf.write(targettype)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_unequip(socket: &mut Socket, slot: u16) -> ClientResult<()> {
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
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::SwitchInvSlot)?;
    buf.write(oldslot)?;
    buf.write(newslot)?;
    buf.write(amount)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_pickup(socket: &mut Socket) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(4)?;

    buf.write(ClientPacket::PickUp)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_dropitem(
    socket: &mut Socket,
    slot: u16,
    amount: u16,
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::DropItem)?;
    buf.write(slot)?;
    buf.write(amount)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_deleteitem(socket: &mut Socket, slot: u16) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::DeleteItem)?;
    buf.write(slot)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_switchstorageslot(
    socket: &mut Socket,
    oldslot: u16,
    newslot: u16,
    amount: u16,
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::SwitchStorageSlot)?;
    buf.write(oldslot)?;
    buf.write(newslot)?;
    buf.write(amount)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_deletestorageitem(
    socket: &mut Socket,
    slot: u16,
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::DeleteStorageItem)?;
    buf.write(slot)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_deposititem(
    socket: &mut Socket,
    inv_slot: u16,
    bank_slot: u16,
    amount: u16,
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::DepositItem)?;
    buf.write(inv_slot)?;
    buf.write(bank_slot)?;
    buf.write(amount)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_withdrawitem(
    socket: &mut Socket,
    inv_slot: u16,
    bank_slot: u16,
    amount: u16,
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::WithdrawItem)?;
    buf.write(inv_slot)?;
    buf.write(bank_slot)?;
    buf.write(amount)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_message(
    socket: &mut Socket,
    channel: MessageChannel,
    msg: String,
    name: String,
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(128)?;

    buf.write(ClientPacket::Message)?;
    buf.write(channel)?;
    buf.write(msg)?;
    buf.write(name)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_admincommand(
    socket: &mut Socket,
    command: AdminCommand,
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(262)?;

    buf.write(ClientPacket::AdminCommand)?;
    buf.write(command)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_settarget(
    socket: &mut Socket,
    entity: Option<Entity>,
) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(12)?;

    buf.write(ClientPacket::SetTarget)?;
    buf.write(entity)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_closestorage(socket: &mut Socket) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(4)?;

    buf.write(ClientPacket::CloseStorage)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_closeshop(socket: &mut Socket) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(4)?;

    buf.write(ClientPacket::CloseShop)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}

pub fn send_closetrade(socket: &mut Socket) -> ClientResult<()> {
    let mut buf = ByteBuffer::new_packet_with(4)?;

    buf.write(ClientPacket::CloseTrade)?;
    buf.finish()?;

    socket.send(buf);
    Ok(())
}
