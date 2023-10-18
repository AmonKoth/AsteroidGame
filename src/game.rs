use std::collections::HashMap;

use sdl2::rect::Point;
use specs::{Builder, Join, World, WorldExt};
const PLAYER_MOVE_SPEED: i32 = 5;

use crate::components;
pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
    let mut players = ecs.write_storage::<crate::components::Player>();
    for player in (&mut players).join() {
        if crate::utils::is_key_pressed(&key_manager, "D") {
            player.direction = components::Direction::Right;
            player.speed = PLAYER_MOVE_SPEED;
        } else if crate::utils::is_key_pressed(&key_manager, "A") {
            player.direction = components::Direction::Left;
            player.speed = PLAYER_MOVE_SPEED;
        } else if crate::utils::is_key_pressed(&key_manager, "W") {
            player.direction = components::Direction::Up;
            player.speed = PLAYER_MOVE_SPEED;
        } else if crate::utils::is_key_pressed(&key_manager, "S") {
            player.direction = components::Direction::Down;
            player.speed = PLAYER_MOVE_SPEED;
        } else {
            player.speed = 0;
        }
    }
}
pub fn update_player_rotation(ecs: &mut World, mouse_position: Point) {
    let mut rotations = ecs.write_storage::<crate::components::Position>();
    let players = ecs.read_component::<crate::components::Player>();
    for (rotation, player) in (&mut rotations, &players).join() {}
}

pub fn load_world(ecs: &mut World) {
    ecs.create_entity()
        .with(crate::components::Position {
            pos: Point::new(0, 0),
            rot: 0.0,
        })
        .with(crate::components::Renderable {
            texture_name: String::from("assets/marco.png"),
            input_width: 32,
            input_height: 42,
            output_width: 32,
            output_height: 42,
            frame: 0,
            total_frames: 1,
            rotation: 0.0,
        })
        .with(crate::components::Player {
            speed: 0,
            direction: components::Direction::Right,
        })
        .build();
}
