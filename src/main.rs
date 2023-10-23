use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use specs::{DispatcherBuilder, Join, World, WorldExt};
use std::collections::HashMap;
use std::time::Duration;

pub mod asteroid;
pub mod components;
pub mod game;
pub mod rocket;
pub mod utils;

// const IMAGE_WIDTH: u32 = 32;
// const IMAGE_HEIGHT: u32 = 42;
// const OUTPUTH_WIDTH: u32 = 100;
// const OUTPUTH_HEIGHT: u32 = 100;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture_creator: &TextureCreator<WindowContext>,
    ecs: &World,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let positions = ecs.read_storage::<components::Position>();
    let mut renderables = ecs.write_storage::<components::Renderable>();

    for (renderable, position) in (&mut renderables, &positions).join() {
        let screen_rect = Rect::from_center(
            position.pos,
            renderable.input_width,
            renderable.input_height,
        );
        let texture = texture_creator.load_texture(&renderable.texture_name)?;
        let src = Rect::new(
            (renderable.input_width * renderable.frame) as i32,
            0,
            renderable.input_width,
            renderable.input_height,
        );
        canvas.copy_ex(
            &texture,
            src,
            screen_rect,
            renderable.render_rotation,
            None,
            false,
            false,
        )?;
        renderable.frame = (renderable.frame + 1) % renderable.total_frames;
    }
    // let (width, height) = canvas.output_size()?;
    // let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    // let screen_rect = Rect::from_center(
    //     screen_position,
    //     player.sprite.width(),
    //     player.sprite.height(),
    // );

    // canvas.copy(texture, player.sprite, screen_rect)?;
    // canvas.copy_ex(
    //     texture,
    //     player.sprite,
    //     screen_rect,
    //     player.rotation,
    //     None,
    //     false,
    //     false,
    // )?;

    canvas.present();
    Ok(())
}

fn update_player(ecs: &World) {
    use components::Direction::*;
    let players = ecs.read_storage::<components::Player>();
    let mut positions = ecs.write_storage::<components::Position>();

    for (player, position) in (&players, &mut positions).join() {
        match player.direction {
            Left => {
                position.pos = position.pos.offset(-player.speed, 0);
            }
            Right => {
                position.pos = position.pos.offset(player.speed, 0);
            }
            Up => {
                position.pos = position.pos.offset(0, -player.speed);
            }
            Down => {
                position.pos = position.pos.offset(0, player.speed);
            }
        }
        if position.pos.x > SCREEN_WIDTH {
            position.pos.x -= SCREEN_WIDTH;
        }
        if position.pos.x < 0 {
            position.pos.x += SCREEN_WIDTH;
        }
        if position.pos.y > SCREEN_HEIGHT {
            position.pos.y -= SCREEN_HEIGHT;
        }
        if position.pos.y < 0 {
            position.pos.y += SCREEN_HEIGHT;
        }
    }
}

struct State {
    ecs: World,
}

fn main() -> Result<(), String> {
    println!("Starting Astroids Game");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // let image_context = sdl2::image::init(InitFlag::JPG | InitFlag::PNG)?;

    // let src = Rect::new(0, 0, IMAGE_WIDTH, IMAGE_HEIGHT);

    // let x: i32 = (SCREEN_WIDTH) / 2 as i32;
    // let y: i32 = (SCREEN_HEIGHT) / 2 as i32;

    // let dst = Rect::new(
    //     x - ((OUTPUTH_WIDTH / 2) as i32),
    //     y - ((OUTPUTH_HEIGHT / 2) as i32),
    //     OUTPUTH_WIDTH,
    //     OUTPUTH_HEIGHT,
    //  );
    // let center = Point::new((SCREEN_WIDTH / 2) as i32, (SCREEN_HEIGHT / 2) as i32);

    let window = video_subsystem
        .window("Astroids", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .expect("Failed to crete window Subsytem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create Canvas");

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;
    let mut mouse_pos = Point::new(0, 0);
    let mut key_manager: HashMap<String, bool> = HashMap::new();

    let mut game_state = State { ecs: World::new() };

    game_state.ecs.register::<components::Position>();
    game_state.ecs.register::<components::Renderable>();
    game_state.ecs.register::<components::Player>();
    game_state.ecs.register::<components::Asteroid>();
    game_state.ecs.register::<components::Rocket>();

    let mut dispacher = DispatcherBuilder::new()
        .with(asteroid::AsteroidMover, "asteroid_mover", &[])
        .with(asteroid::AstroidCollider, "asteroid_collider", &[])
        .with(rocket::RocketMover, "rocket_mover", &[])
        .build();

    game::load_world(&mut game_state.ecs);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown { keycode, .. } => match keycode {
                    None => {}
                    Some(key) => {
                        utils::key_down(&mut key_manager, key.to_string());
                    }
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    None => {}
                    Some(key) => {
                        utils::key_up(&mut key_manager, key.to_string());
                    }
                },
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos.x = x;
                    mouse_pos.y = y;
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        println!("FIRE");
                    }
                }

                _ => {}
            }
        }
        update_player(&mut game_state.ecs);
        game::update(&mut game_state.ecs, &mut key_manager);
        game::update_player_rotation(&mut game_state.ecs, mouse_pos);
        // let angle = calculate_agnle(player.position, mouse_pos);
        // player.rotation = angle;
        dispacher.dispatch(&game_state.ecs);
        game_state.ecs.maintain();
        render(
            &mut canvas,
            Color::RGB(0, 0, 0),
            &texture_creator,
            &game_state.ecs,
        )?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
