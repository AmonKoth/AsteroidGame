use specs::prelude::*;
use specs::{Entities, Join};

use crate::components;

pub struct RocketMover;

impl<'a> System<'a> for RocketMover {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        ReadStorage<'a, components::Rocket>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, mut renderables, rockets, entities) = data;

        for (position, render, rocket, entity) in
            (&mut positions, &mut renderables, &rockets, &entities).join()
        {
            let radian = position.rot.to_radians();

            let move_x = rocket.speed * radian.sin();
            let move_y = rocket.speed * radian.cos();

            position.pos.x += move_x as i32;
            position.pos.y -= move_y as i32;
            if position.pos.x > crate::SCREEN_WIDTH.into()
                || position.pos.x < 0
                || position.pos.y > crate::SCREEN_HEIGHT.into()
                || position.pos.y < 0
            {
                entities.delete(entity).ok();
            }
            render.render_rotation = position.rot;
        }
    }
}

pub struct RocketDamage;

impl<'a> System<'a> for RocketDamage {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Rocket>,
        WriteStorage<'a, components::Asteroid>,
        WriteStorage<'a, components::Player>,
        WriteStorage<'a, components::GameData>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderers, rockets, asteroids, _players, _, entities) = &data;
        let mut asteroid_creation = Vec::<components::PendingAsteroid>::new();
        let mut score: u32 = 0;

        for (rocket_pos, _, _, rocket_entity) in (positions, renderers, rockets, entities).join() {
            for (asteroid_pos, asteroid_render, asteroid, asteroid_entity) in
                (positions, renderers, asteroids, entities).join()
            {
                let diff_x: f64 = ((rocket_pos.pos.x - asteroid_pos.pos.x) as f64).abs();
                let diff_y: f64 = ((rocket_pos.pos.y - asteroid_pos.pos.y) as f64).abs();
                let hyp: f64 = ((diff_x * diff_x) + (diff_y * diff_y)).sqrt();
                if hyp < asteroid_render.output_width as f64 / 2.0 {
                    score += asteroid.size_multiplier;
                    entities.delete(asteroid_entity).ok();
                    entities.delete(rocket_entity).ok();
                    if asteroid.size_multiplier > 1 {
                        asteroid_creation.push(components::PendingAsteroid {
                            position: asteroid_pos.pos,
                            rot: asteroid_pos.rot - 90.0,
                            size_mult: asteroid.size_multiplier / 2,
                        });
                        asteroid_creation.push(components::PendingAsteroid {
                            position: asteroid_pos.pos,
                            rot: asteroid_pos.rot + 90.0,
                            size_mult: asteroid.size_multiplier / 2,
                        });
                    }
                }
            }
        }

        let (mut positions, mut renderers, _, mut asteroids, _, _, entities) = data;
        for new_asteroid in asteroid_creation {
            let new_ast = entities.create();
            positions
                .insert(
                    new_ast,
                    components::Position {
                        pos: new_asteroid.position,
                        rot: new_asteroid.rot,
                    },
                )
                .ok();
            asteroids
                .insert(
                    new_ast,
                    components::Asteroid {
                        speed: 6.0,
                        rotation_speed: 2.0,
                        size_multiplier: new_asteroid.size_mult,
                    },
                )
                .ok();
            renderers
                .insert(
                    new_ast,
                    components::Renderable {
                        texture_name: String::from("assets/running.png"),
                        input_width: 32,
                        input_height: 42,
                        output_width: 32 * new_asteroid.size_mult,
                        output_height: 42 * new_asteroid.size_mult,
                        frame: 1,
                        total_frames: 9,
                        render_rotation: 0.0,
                    },
                )
                .ok();
        }
        let (_, _, _, _, _, mut gamedatas, _) = data;
        for mut gamedata in (&mut gamedatas).join() {
            gamedata.score += score;
        }
    }
}
