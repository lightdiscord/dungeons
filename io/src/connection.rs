use crate::Serializer;
use tokio::sync::mpsc::UnboundedSender;
use serde::Serialize;
use bytes::Bytes;

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

    // TODO: Result return type
    pub fn send<P>(&self, item: &P)
    where
        P: Serialize
    {
        let mut serializer = Serializer::default();
        serializer.serialize(item).unwrap();
        self.tx.send(serializer.into()).unwrap();
    }
}
