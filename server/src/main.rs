#![feature(try_trait)]

use protocol::{Packet, VarInt, HandshakingPacket, NextState};

use bytes::{Buf, BytesMut};

use std::io;

use tokio::stream::StreamExt;
use tokio::net::{TcpStream, TcpListener};

use tokio_util::codec::{ Encoder, Decoder };

const ADDRESS: &'static str = "127.0.0.1:25565";

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    Utf8Error(std::string::FromUtf8Error),
    NoneError(std::option::NoneError)
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::Utf8Error(error)
    }
}

impl From<std::option::NoneError> for Error {
    fn from(error: std::option::NoneError) -> Self {
        Error::NoneError(error)
    }
}

struct MinecraftCodec {
    next_size: Option<usize>,
}

impl Default for MinecraftCodec {
    fn default() -> Self {
        MinecraftCodec {
            next_size: None
        }
    }
}

trait BufRead: Sized {
    type Error;

    fn read<B: Buf>(src: &mut B) -> Result<Option<Self>, Self::Error>;
}

impl BufRead for VarInt {
    type Error = io::Error;

    fn read<B: Buf>(src: &mut B) -> Result<Option<Self>, Self::Error> {
        let mut idx = 0;
        let mut result: i32 = 0;

        loop {
            if src.remaining() < idx + 1 { return Ok(None) }

            let byte = src.bytes()[idx];
            let value = byte & 0b01111111;
            result |= (value as i32) << (7 * idx);
            idx += 1;

            if idx > 5 { return Err(io::ErrorKind::InvalidData.into()); }
            if (byte & 0b10000000) == 0 { break; }
        }

        src.advance(idx);

        Ok(Some(VarInt(result)))
    }
}

impl BufRead for String {
    type Error = Error;

    fn read<B: Buf>(src: &mut B) -> Result<Option<Self>, Self::Error> {
        let length = *VarInt::read(src)?? as usize;
        let mut buffer = vec![0; length];
        src.copy_to_slice(&mut buffer);
        
        Ok(Some(String::from_utf8(buffer)?))
    }
}

impl BufRead for HandshakingPacket {
    type Error = Error;

    fn read<B: Buf>(src: &mut B) -> Result<Option<Self>, Self::Error> {
        let protocol_version = VarInt::read(src)??;
        let server_address = String::read(src)??;
        let server_port = src.get_u16();
        let next_state = match *VarInt::read(src)?? {
            1 => NextState::Status,
            2 => NextState::Login,
            _ => Err(io::Error::from(io::ErrorKind::InvalidData))?
        };

        Ok(Some(HandshakingPacket::Handshake {
            protocol_version,
            server_address,
            server_port,
            next_state
        }))
    }
}

impl Decoder for MinecraftCodec {
    type Item = Packet;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Error> {
        // Read the size of the packet
        if self.next_size.is_none() {
            self.next_size = VarInt::read(src)?.map(|n| n.0 as usize);
        }

        // If `src` is too small or no size was read, return nothing.
        if self.next_size.map(|size| size > src.remaining()).unwrap_or(true) {
            return Ok(None);
        }

        let packet_id = *VarInt::read(src)?? as usize;
        let packet = match packet_id {
            0x00 => Packet::Handshaking(HandshakingPacket::read(src)??),
            _ => unimplemented!()
        };

        Ok(Some(packet))
    }
}

impl Encoder for MinecraftCodec {
    type Item = Packet;
    type Error = io::Error;

    fn encode(&mut self, _item: Self::Item, _dest: &mut BytesMut) -> io::Result<()> {
        unimplemented!()
    }
}

async fn process_stream(stream: TcpStream) -> Result<(), Error> {
    let mut framed = MinecraftCodec::default().framed(stream);

    while let Some(message) = framed.next().await {
        match message {
            Ok(bytes) => println!("bytes: {:?}", bytes),
            Err(error) => {
                eprintln!("socket closed with error: {:?}", error);
                return Err(error);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind(ADDRESS).await?;


    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async { process_stream(socket).await });
    }
}
