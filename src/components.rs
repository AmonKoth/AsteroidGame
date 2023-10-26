use specs::prelude::*;
use specs_derive::Component;

use sdl2::rect::Point;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Position {
    pub pos: Point,
    pub rot: f64,
}
#[derive(Component)]
pub struct Renderable {
    pub texture_name: String,
    pub input_width: u32,
    pub input_height: u32,
    pub output_width: u32,
    pub output_height: u32,
    pub frame: u32,
    pub total_frames: u32,
    pub render_rotation: f64,
}

#[derive(Component)]
pub struct Player {
    pub speed: i32,
    pub direction: Direction,
    pub can_take_damage: bool,
}
#[derive(Component)]
pub struct Asteroid {
    pub speed: f64,
    pub rotation_speed: f64,
    pub size_multiplier: u32,
}
#[derive(Component)]
pub struct Rocket {
    pub speed: f64,
}

pub struct PendingAsteroid {
    pub position: Point,
    pub rot: f64,
    pub size_mult: u32,
}

#[derive(Component)]
pub struct GameData {
    pub score: u32,
    pub level: u32,
}
