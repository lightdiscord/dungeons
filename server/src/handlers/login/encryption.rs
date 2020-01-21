use crate::handlers::Handler;
use io::connection::Connection;
use protocol::packets::login as login_packets;
use login_packets::serverbound::EncryptionResponse;
use failure::Fallible;

impl Handler for EncryptionResponse {
    type Context = Connection;

    fn handle(&mut self, _connection: &mut Self::Context) -> Fallible<()> {
        unimplemented!()
    }
}

