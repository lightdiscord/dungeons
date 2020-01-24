use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid next state")]
    InvalidNextState,

    #[fail(display = "invalid chat mode")]
    InvalidChatMode,

    #[fail(display = "invalid main hand")]
    InvalidMainHand,
}

