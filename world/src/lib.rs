use legion::prelude::*;

pub use legion;

#[derive(Debug)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32
}

pub fn create_world() -> World {
    let universe = Universe::new();
    universe.create_world()
}

