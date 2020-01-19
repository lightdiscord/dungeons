use crate::Serializer;
use tokio::sync::mpsc::UnboundedSender;
use serde::Serialize;
use bytes::Bytes;
use failure::Fallible;

pub enum ConnectionState {
    Handshaking,
    Login,
    Status
}

impl Default for ConnectionState {
    fn default() -> Self { ConnectionState::Handshaking }
}

pub struct Connection {
    pub state: ConnectionState,
    tx: UnboundedSender<Bytes>
}

impl Connection {
    pub fn new(tx: UnboundedSender<Bytes>) -> Self {
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
        self.tx.send(serializer.into())?;
        Ok(())
    }
}
