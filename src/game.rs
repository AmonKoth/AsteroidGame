use std::collections::HashMap;

use sdl2::rect::Point;
use specs::{Builder, Join, World, WorldExt};
const PLAYER_MOVE_SPEED: i32 = 5;

use crate::components;
pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
    let mut must_reload_world = false;
    {
        let players = ecs.read_storage::<crate::components::Player>();
        if players.join().count() < 1 {
            must_reload_world = true;
        }
    }

    if must_reload_world {
        ecs.delete_all();
        load_world(ecs);
    }

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
    let mut renderables = ecs.write_storage::<crate::components::Renderable>();
    let players = ecs.read_component::<crate::components::Player>();
    let positions = ecs.read_component::<crate::components::Position>();
    for (renderable, _, position) in (&mut renderables, &players, &positions).join() {
        let delta_x = (mouse_position.x - position.pos.x) as f64;
        let delta_y = (mouse_position.y - position.pos.y) as f64;
        let angle = delta_y.atan2(delta_x);

        let angle_degrees = angle.to_degrees();
        renderable.render_rotation = angle_degrees
    }
}

pub fn load_world(ecs: &mut World) {
    ecs.create_entity()
        .with(crate::components::Position {
            pos: Point::new(50, 50),
            rot: 270.0,
        })
        .with(crate::components::Renderable {
            texture_name: String::from("assets/marco.png"),
            input_width: 32,
            input_height: 42,
            output_width: 32,
            output_height: 42,
            frame: 0,
            total_frames: 1,
            render_rotation: 0.0,
        })
        .with(crate::components::Player {
            speed: 0,
            direction: components::Direction::Right,
        })
        .build();
    ecs.create_entity()
        .with(crate::components::Position {
            pos: Point::new(200, 400),
            rot: 45.0,
        })
        .with(crate::components::Renderable {
            texture_name: String::from("assets/running.png"),
            input_width: 25,
            input_height: 45,
            output_width: 25,
            output_height: 45,
            frame: 0,
            total_frames: 1,
            render_rotation: 0.0,
        })
        .with(crate::components::Asteroid {
            speed: 4.0,
            rotation_speed: 0.5,
        })
        .build();
}
