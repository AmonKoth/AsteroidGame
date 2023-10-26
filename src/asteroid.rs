use specs::prelude::Entities;
use specs::{Join, System, WriteStorage};

pub struct AsteroidMover;

use crate::components;

impl<'a> System<'a> for AsteroidMover {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Asteroid>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        for (position, render, asteriod) in (&mut data.0, &mut data.1, &data.2).join() {
            let radians = position.rot.to_radians();

            position.pos.x += (asteriod.speed * radians.sin()) as i32;
            position.pos.y -= (asteriod.speed * radians.cos()) as i32;

            let half_width = (render.output_width / 2) as i32;
            let half_height = (render.output_height / 2) as i32;

            if position.pos.x > (crate::SCREEN_WIDTH - half_width).into()
                || position.pos.x < half_width.into()
            {
                position.rot = 360.0 - position.rot;
            } else if position.pos.y > (crate::SCREEN_HEIGHT - half_height).into()
                || position.pos.y < half_height.into()
            {
                if position.rot > 180.0 {
                    position.rot = 540.0 - position.rot;
                } else {
                    position.rot = 180.0 - position.rot;
                }
            }

            render.render_rotation += asteriod.speed;
            if render.render_rotation > 360.0 {
                render.render_rotation -= 360.0;
            }
            if render.render_rotation < 0.0 {
                render.render_rotation += 360.0;
            }
        }
    }
}

pub struct AstroidCollider;
impl<'a> System<'a> for AstroidCollider {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        WriteStorage<'a, components::Renderable>,
        WriteStorage<'a, components::Player>,
        WriteStorage<'a, components::Asteroid>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, render, players, asteroids, entites) = data;

        for player in (players).join() {
            if !player.can_take_damage {
                return;
            }
        }
        for (player_pos, player_render, _, entity) in
            (&positions, &render, &players, &entites).join()
        {
            for (asteroid_pos, asteroid_rend, _) in (&positions, &render, &asteroids).join() {
                let diff_x: f64 = ((player_pos.pos.x - asteroid_pos.pos.x) as f64).abs();
                let diff_y: f64 = ((player_pos.pos.y - asteroid_pos.pos.y) as f64).abs();
                let hyp: f64 = ((diff_x * diff_x) + (diff_y * diff_y)).sqrt();

                if hyp < (asteroid_rend.output_width + player_render.output_width) as f64 / 2.0 {
                    println!("Player Died");
                    entites.delete(entity).ok();
                }
            }
        }
    }
}
