use std::convert::TryFrom;
use io::packets;
use io::types::{MaxedString, Var};
use serde::Deserialize;
use crate::error::Error;

#[derive(Debug, Deserialize)]
#[serde(try_from = "Var<i32>")]
pub enum ChatMode {
    Enabled,
    CommandOnly,
    Hidden
}

impl TryFrom<Var<i32>> for ChatMode {
    type Error = Error;

    fn try_from(src: Var<i32>) -> Result<Self, Self::Error> {
        match *src {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandOnly),
            2 => Ok(ChatMode::Hidden),
            _ => Err(Error::InvalidChatMode)
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "u8")]
pub struct SkinParts {
    cape: bool,
    jacket: bool,
    left_sleeve: bool,
    right_sleeve: bool,
    left_leg: bool,
    right_leg: bool,
    hat: bool
}

impl From<u8> for SkinParts {
    fn from(src: u8) -> Self {
        SkinParts {
            cape: (src & 0x1) != 0,
            jacket: (src & 0x2) != 0,
            left_sleeve: (src & 0x4) != 0,
            right_sleeve: (src & 0x8) != 0,
            left_leg: (src & 0x10) != 0,
            right_leg: (src & 0x20) != 0,
            hat: (src & 0x40) != 0
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "Var<i32>")]
pub enum MainHand {
    Left,
    Right
}

impl TryFrom<Var<i32>> for MainHand {
    type Error = Error;

    fn try_from(src: Var<i32>) -> Result<Self, Self::Error> {
        match *src {
            0 => Ok(MainHand::Left),
            1 => Ok(MainHand::Right),
            _ => Err(Error::InvalidMainHand)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientSettings {
    locale: MaxedString<16>,
    view_distance: u8,
    chat_mode: ChatMode,
    colors: bool,
    displayed_skin_parts: SkinParts,
    main_hand: MainHand
}

#[derive(Debug, Deserialize)]
pub struct PluginMessage {
    identifier: MaxedString<32767>
    // TODO: Create a rest bytes type;
}

#[derive(Debug, Deserialize)]
pub struct TeleportConfirm {
    teleport_id: Var<i32>
}

#[derive(Debug, Deserialize)]
pub struct PlayerPositionAndLook {
    x: f64,
    feet_y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool
}

packets! {
    serverbound {
        0x05 => ClientSettings,
        0x0b => PluginMessage,
        0x00 => TeleportConfirm,
        0x12 => PlayerPositionAndLook
    }
}
