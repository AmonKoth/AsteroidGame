use std::collections::HashMap;

use rand::Rng;
use sdl2::rect::Point;
use specs::{Builder, Join, World, WorldExt};

const PLAYER_MOVE_SPEED: i32 = 5;
const MAX_MISSILES: usize = 5;

use crate::{components, SCREEN_HEIGHT, SCREEN_WIDTH, X_GRID_COUNT, Y_GRID_COUNT};

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
    let mut must_reload_world = false;
    let mut create_asteroids_pressed = false;
    let mut current_player_pos = components::Position {
        pos: Point::new(0, 0),
        rot: 0.0,
    };
    {
        let players = ecs.read_storage::<crate::components::Player>();
        let positions = ecs.read_storage::<crate::components::Position>();

        for (position, _player) in (&positions, &players).join() {
            current_player_pos.pos.x = position.pos.x;
            current_player_pos.pos.y = position.pos.y;
        }
        if players.join().count() < 1 {
            must_reload_world = true;
        }
    }

    if must_reload_world {
        ecs.delete_all();
        load_world(ecs);
    }

    let mut must_create_astroids = false;
    let mut number_asteroids: u32 = 0;
    {
        let asteroids = ecs.read_storage::<components::Asteroid>();
        if asteroids.count() < 1 && !create_asteroids_pressed {
            must_create_astroids = true;
            let mut gamedatas = ecs.write_storage::<components::GameData>();
            for mut gamedata in (&mut gamedatas).join() {
                gamedata.level += 1;
                number_asteroids = (gamedata.level / 3) + 1;
            }
        }
        if asteroids.count() as u32 == number_asteroids {
            create_asteroids_pressed = false;
        }
    }
    if crate::utils::is_key_pressed(&key_manager, "V") {
        crate::utils::key_up(key_manager, "V".to_string());
        number_asteroids += 20000;
        create_asteroids_pressed = true;
    }
    if must_create_astroids || create_asteroids_pressed {
        let mut asteroid_count: u32 = 0;
        while asteroid_count < number_asteroids {
            let mut rng = rand::thread_rng();
            let size = rng.gen_range(1..6);
            let next_x = rng.gen_range(50..SCREEN_WIDTH - 50);
            let next_y = rng.gen_range(50..SCREEN_HEIGHT - 50);
            let grid_x = X_GRID_COUNT * (next_x / 100);
            let grid_y = Y_GRID_COUNT * (next_y / 100);
            let next_rot = rng.gen_range(0.0..360.0);

            let diff_x = ((current_player_pos.pos.x - next_x) as f64).abs();
            let diff_y = ((current_player_pos.pos.y - next_y) as f64).abs();
            if ((diff_x * diff_x) + (diff_y * diff_y)).sqrt() < 150.0 {
                continue;
            }
            asteroid_count += 1;
            let new_asteroid = components::Position {
                pos: Point::new(next_x, next_y),
                rot: next_rot,
            };
            let new_collider = components::Collider {
                grid_x: grid_x,
                grid_y: grid_y,
            };

            create_asteroid(ecs, new_asteroid, size, new_collider);
        }
    }

    let mut player_position = components::Position {
        pos: Point::new(0, 0),
        rot: 0.0,
    };

    let mut must_fire_rocket = false;
    let mut player_grid_location = components::Collider {
        grid_x: 0,
        grid_y: 0,
    };
    {
        let mut players = ecs.write_storage::<crate::components::Player>();
        let positions = ecs.read_storage::<crate::components::Position>();
        let grid_location = ecs.read_storage::<crate::components::Collider>();
        for (player, position, grid) in (&mut players, &positions, &grid_location).join() {
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
            if crate::utils::is_key_pressed(&key_manager, "C") {
                crate::utils::key_up(key_manager, "C".to_string());
                player.can_take_damage = !player.can_take_damage;
                println!(
                    "Player can take damage: {}",
                    player.can_take_damage.to_string()
                );
            }

            if crate::utils::is_key_pressed(&key_manager, "Space") {
                must_fire_rocket = true;
                crate::utils::key_up(key_manager, "Space".to_string());
                player_position.pos.x = position.pos.x;
                player_position.pos.y = position.pos.y;
                player_grid_location.grid_x = grid.grid_x;
                player_grid_location.grid_y = grid.grid_y;
                player_position.rot = position.rot + 90.0; // +90 cause player sprite is looking at the side
            } else {
                must_fire_rocket = false;
            }
        }
    }

    if must_fire_rocket {
        fire_rocket(ecs, player_position, player_grid_location);
    }
}
pub fn update_player_rotation(ecs: &mut World, mouse_position: Point) {
    let mut renderables = ecs.write_storage::<crate::components::Renderable>();
    let players = ecs.read_component::<crate::components::Player>();
    let mut positions = ecs.write_component::<crate::components::Position>();
    for (renderable, _, position) in (&mut renderables, &players, &mut positions).join() {
        let delta_x = (mouse_position.x - position.pos.x) as f64;
        let delta_y = (mouse_position.y - position.pos.y) as f64;
        let angle = delta_y.atan2(delta_x);

        let angle_degrees = angle.to_degrees();
        renderable.render_rotation = angle_degrees;
        position.rot = renderable.render_rotation;
    }
}

pub fn load_world(ecs: &mut World) {
    ecs.create_entity()
        .with(components::Position {
            pos: Point::new(50, 50),
            rot: 270.0,
        })
        .with(components::Renderable {
            texture_name: String::from("marco"),
            input_width: 32,
            input_height: 42,
            output_width: 32,
            output_height: 42,
            frame: 1,
            total_frames: 9,
            render_rotation: 0.0,
        })
        .with(components::Player {
            speed: 0,
            direction: components::Direction::Right,
            can_take_damage: true,
        })
        .with(components::Collider {
            grid_x: 0,
            grid_y: 0,
        })
        .build();
    create_asteroid(
        ecs,
        components::Position {
            pos: Point::new(200, 400),
            rot: 45.0,
        },
        2,
        components::Collider {
            grid_x: X_GRID_COUNT * 2,
            grid_y: Y_GRID_COUNT * 4,
        },
    );
    ecs.create_entity()
        .with(components::GameData { score: 0, level: 1 })
        .build();
}

fn fire_rocket(ecs: &mut World, position: components::Position, collider: components::Collider) {
    {
        let rockets = ecs.read_storage::<crate::components::Rocket>();
        if rockets.count() > MAX_MISSILES - 1 {
            return;
        }
    }
    ecs.create_entity()
        .with(position)
        .with(components::Renderable {
            texture_name: String::from("rocket"),
            input_width: 17,
            input_height: 61,
            output_width: 40,
            output_height: 61,
            frame: 0,
            total_frames: 1,
            render_rotation: 0.0,
        })
        .with(collider)
        .with(components::Rocket { speed: 10.0 })
        .build();
}

fn create_asteroid(
    ecs: &mut World,
    position: components::Position,
    asteroid_size_mult: u32,
    collider: components::Collider,
) {
    ecs.create_entity()
        .with(position)
        .with(components::Renderable {
            texture_name: String::from("enemy"),
            input_width: 33,
            input_height: 45,
            output_width: 33 * asteroid_size_mult,
            output_height: 45 * asteroid_size_mult,
            frame: 1,
            total_frames: 12,
            render_rotation: 0.0,
        })
        .with(crate::components::Asteroid {
            speed: 4.0,
            rotation_speed: 0.5,
            size_multiplier: asteroid_size_mult,
        })
        .with(collider)
        .build();
}
