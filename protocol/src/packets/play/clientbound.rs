use io::types::Var;
use serde::Serialize;
use io::packets;

#[serde(into = "u64")]
#[derive(Debug, Clone, Serialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Into<u64> for Position {
    fn into(self) -> u64 {
        let Position { x, y, z } = self;

        (x as u64 & 0x3ffffff) << 38 | (y as u64 & 0xfff) << 26 | z as u64 & 0x3ffffff
    }
}

#[derive(Debug, Serialize)]
pub struct SpawnPosition {
    pub location: Position
}

#[serde(into = "u8")]
#[derive(Debug, Clone, Serialize)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator
}

impl Into<u8> for Gamemode {
    fn into(self) -> u8 {
        match self {
            Gamemode::Survival => 0,
            Gamemode::Creative => 1,
            Gamemode::Adventure => 2,
            Gamemode::Spectator => 3
        }
    }
}

#[serde(into = "i32")]
#[derive(Debug, Clone, Serialize)]
pub enum Dimension {
    Nether,
    Overworld,
    End
}

impl Into<i32> for Dimension {
    fn into(self) -> i32 {
        match self {
            Dimension::Nether => -1,
            Dimension::Overworld => 0,
            Dimension::End => 1
        }
    }
}

#[derive(Debug, Serialize)]
pub struct JoinGame {
    pub entity_id: u32,
    pub gamemode: Gamemode,
    pub dimension: Dimension,
    pub hashed_seed: u64,
    pub max_player: u8,
    pub level_type: String,
    pub view_distance: Var<i32>,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool
}

#[derive(Debug, Serialize)]
pub struct PlayerPositionAndLook {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    // TODO: Create a type (useful to know if x/y/z/yaw/pitch are relative or absolute)
    pub flags: u8,
    pub teleport_id: Var<i32>
}

packets! {
    clientbound {
        0x26 => JoinGame,
        0x36 => PlayerPositionAndLook
    }
}
