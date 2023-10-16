use sdl2::event::{self, Event};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use std::collections::HashMap;
use std::time::Duration;

pub mod utils;
pub mod vector2D;

const IMAGE_WIDTH: u32 = 32;
const IMAGE_HEIGHT: u32 = 42;
const OUTPUTH_WIDTH: u32 = 100;
const OUTPUTH_HEIGHT: u32 = 100;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const PLAYER_MOVE_SPEED: i32 = 5;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(
        screen_position,
        player.sprite.width(),
        player.sprite.height(),
    );

    canvas.copy(texture, player.sprite, screen_rect)?;

    canvas.present();
    Ok(())
}

fn update_player(player: &mut Player) {
    use self::Direction::*;
    match player.direction {
        Left => {
            player.position = player.position.offset(-player.speed, 0);
        }
        Right => {
            player.position = player.position.offset(player.speed, 0);
        }
        Up => {
            player.position = player.position.offset(0, -player.speed);
        }
        Down => {
            player.position = player.position.offset(0, player.speed);
        }
    }
}

fn main() -> Result<(), String> {
    println!("Starting Astroids Game");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let image_context = sdl2::image::init(InitFlag::JPG | InitFlag::PNG)?;

    // let src = Rect::new(0, 0, IMAGE_WIDTH, IMAGE_HEIGHT);

    // let x: i32 = (SCREEN_WIDTH) / 2 as i32;
    // let y: i32 = (SCREEN_HEIGHT) / 2 as i32;

    // let dst = Rect::new(
    //     x - ((OUTPUTH_WIDTH / 2) as i32),
    //     y - ((OUTPUTH_HEIGHT / 2) as i32),
    //     OUTPUTH_WIDTH,
    //     OUTPUTH_HEIGHT,
    // );
    // let center = Point::new((OUTPUTH_WIDTH / 2) as i32, (OUTPUTH_HEIGHT / 2) as i32);

    let window = video_subsystem
        .window("Astroids", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to crete window Subsytem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create Canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("assets/marco.png")
        .map_err(|e| e.to_string())?;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 32, 42),
        speed: 5,
        direction: Direction::Right,
    };
    let mut event_pump = sdl_context.event_pump()?;
    let mut key_manager: HashMap<String, bool> = HashMap::new();
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

                _ => {}
            }
            if utils::is_key_pressed(&key_manager, "W") {
                player.direction = Direction::Up;
                player.speed = PLAYER_MOVE_SPEED;
            } else if utils::is_key_pressed(&key_manager, "S") {
                player.direction = Direction::Down;
                player.speed = PLAYER_MOVE_SPEED;
            } else if utils::is_key_pressed(&key_manager, "A") {
                player.direction = Direction::Left;
                player.speed = PLAYER_MOVE_SPEED;
            } else if utils::is_key_pressed(&key_manager, "D") {
                player.direction = Direction::Right;
                player.speed = PLAYER_MOVE_SPEED;
            } else {
                player.speed = 0;
            }
        }
        update_player(&mut player);
        render(&mut canvas, Color::RGB(0, 0, 0), &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
