use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid next state")]
    InvalidNextState
}

