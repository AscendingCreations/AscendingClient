use crate::socket::*;
use bytey::ByteBuffer;
use phf::phf_map;

type PacketFunction = fn(&mut Socket, &mut World, &mut DrawSetting, &mut Content, &mut ByteBuffer) -> Result<()>;

static PACKET_MAP: phf::Map<u32, PacketFunction> = phf_map! {
    0u32 => handle_ping,
    1u32 => handle_status,
    2u32 => handle_alertmsg,
    3u32 => handle_fltalert,
};

pub fn handle_data(socket: &mut Socket, world: &mut World, systems: &mut DrawSetting, content: &mut Content, data: &mut ByteBuffer) -> Result<()> {
    let id: u32 = data.read()?;

    if id > 80 {
        return Err(AscendingError::InvalidPacket);
    }

    let fun = match PACKET_MAP.get(&id) {
        Some(fun) => fun,
        None => return Err(AscendingError::InvalidPacket),
    };

    fun(socket, world, systems, content, data)
}

fn handle_ping(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _data: &mut ByteBuffer
) -> Result<()> {
    Ok(())
}

fn handle_status(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    _data: &mut ByteBuffer
) -> Result<()> {
    Ok(())
}

fn handle_alertmsg(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    data: &mut ByteBuffer
) -> Result<()> {
    let message = data.read::<String>()?;
    let close = data.read::<u8>()?;

    println!("{}, should close: {}", message, close);
    Ok(())
}

fn handle_fltalert(
    _socket: &mut Socket, 
    _world: &mut World,
    _systems: &mut DrawSetting,
    _content: &mut Content,
    data: &mut ByteBuffer
) -> Result<()> {
    let _flttype = data.read::<u8>()?;
    let _message = data.read::<String>()?;
    
    Ok(())
}