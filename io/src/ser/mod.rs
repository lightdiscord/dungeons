use bytes::BytesMut;

mod traits;

#[derive(Default)]
pub struct Serializer(BytesMut);

