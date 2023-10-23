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
