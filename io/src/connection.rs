use crate::Serializer;
use tokio::sync::mpsc::UnboundedSender;
use serde::Serialize;
use bytes::Bytes;
use failure::Fallible;

pub enum ConnectionState {
    Handshaking,
    Login,
    Status,
    Play
}

impl Default for ConnectionState {
    fn default() -> Self { ConnectionState::Handshaking }
}

#[derive(Debug)]
pub enum ConnectionEvent {
    Message(Bytes),
    Close
}

pub struct Connection {
    pub state: ConnectionState,
    tx: UnboundedSender<ConnectionEvent>
}

impl Connection {
    pub fn new(tx: UnboundedSender<ConnectionEvent>) -> Self {
        Connection {
            state: Default::default(),
            tx
        }
    }

    pub fn send<P>(&self, item: &P) -> Fallible<()>
    where
        P: Serialize
    {
        let mut serializer = Serializer::default();
        serializer.serialize(item)?;
        let tmp = serializer.into();
        println!("bytes sent: {:?}", tmp);
        self.tx.send(ConnectionEvent::Message(tmp))?;
        Ok(())
    }

    pub fn close(&self) -> Fallible<()> {
        Ok(self.tx.send(ConnectionEvent::Close)?)
    }
}
