use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::{DispatcherBuilder, Join, World, WorldExt};

use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};

pub mod asteroid;
pub mod components;
pub mod game;
pub mod rocket;
pub mod texture_manager;
pub mod utils;

// const IMAGE_WIDTH: u32 = 32;
// const IMAGE_HEIGHT: u32 = 42;
// const OUTPUTH_WIDTH: u32 = 100;
// const OUTPUTH_HEIGHT: u32 = 100;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const GRID_SIZE: i32 = 100;
const X_GRID_COUNT: i32 = SCREEN_WIDTH / GRID_SIZE;
const Y_GRID_COUNT: i32 = SCREEN_HEIGHT / GRID_SIZE;

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture_manager: &texture_manager::TextureManager,
    ui_elements: &Vec<UIElement>,
    ecs: &World,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let positions = ecs.read_storage::<components::Position>();
    let mut renderables = ecs.write_storage::<components::Renderable>();

    for (renderable, position) in (&mut renderables, &positions).join() {
        let screen_rect = Rect::from_center(
            position.pos,
            renderable.output_width,
            renderable.output_height,
        );
        let texture = texture_manager.get_texture(&renderable.texture_name)?;
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

    for ui_element in ui_elements {
        canvas.copy(&ui_element.texture, None, Some(ui_element.position))?;
    }

    canvas.present();
    Ok(())
}

fn update_player(ecs: &World) {
    use components::Direction::*;
    let players = ecs.read_storage::<components::Player>();
    let mut collisions = ecs.write_storage::<components::Collider>();
    let mut positions = ecs.write_storage::<components::Position>();

    for (player, position, collider) in (&players, &mut positions, &mut collisions).join() {
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
        collider.grid_x = X_GRID_COUNT * (position.pos.x / 100);
        collider.grid_y = Y_GRID_COUNT * (position.pos.y / 100);
    }
}

struct State {
    ecs: World,
}

struct UIElement<'a> {
    texture: Texture<'a>,
    position: Rect,
}

fn main() -> Result<(), String> {
    println!("Starting Astroids Game");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Astroids", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .borderless()
        .build()
        .expect("Failed to crete window Subsytem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create Canvas");

    let texture_creator = canvas.texture_creator();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"assets/Fonts/airstrikeexpand.ttf");
    let mut texture_manager = texture_manager::TextureManager::new(&texture_creator);
    texture_manager.load_texture(&String::from("marco"), &String::from("assets/marco.png"))?;
    texture_manager.load_texture(&String::from("enemy"), &String::from("assets/running.png"))?;
    texture_manager.load_texture(&String::from("rocket"), &String::from("assets/rocket.png"))?;

    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut event_pump = sdl_context.event_pump()?;
    let mut mouse_pos = Point::new(0, 0);
    let mut key_manager: HashMap<String, bool> = HashMap::new();

    let mut game_state = State { ecs: World::new() };

    game_state.ecs.register::<components::Position>();
    game_state.ecs.register::<components::Renderable>();
    game_state.ecs.register::<components::Player>();
    game_state.ecs.register::<components::Asteroid>();
    game_state.ecs.register::<components::Rocket>();
    game_state.ecs.register::<components::GameData>();
    game_state.ecs.register::<components::Collider>();

    let mut dispacher = DispatcherBuilder::new()
        .with(asteroid::AsteroidMover, "asteroid_mover", &[])
        .with(asteroid::AstroidCollider, "asteroid_collider", &[])
        .with(rocket::RocketMover, "rocket_mover", &[])
        .with(rocket::RocketDamage, "rocket_damage", &[])
        .build();

    game::load_world(&mut game_state.ecs);

    //FPS counter
    let mut frame_count = 0;
    let mut last_second = Instant::now();

    let max_ui_render_wait = 100;
    let mut ui_render_wait = max_ui_render_wait;
    let mut ui_storage: Vec<UIElement> = Vec::new();

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
        frame_count += 1;
        let elapsed_time = last_second.elapsed().as_secs_f64();

        if elapsed_time >= 1.0 {
            let fps = frame_count as f64 / elapsed_time;
            println!("FPS :{:.2}", fps);
            frame_count = 0;
            last_second = Instant::now();
        }
        ui_render_wait += 1;
        if ui_render_wait >= max_ui_render_wait {
            ui_render_wait = 0;
            ui_storage.clear();
            {
                let players = game_state.ecs.read_storage::<crate::components::Player>();
                for player in (players).join() {
                    let immortality_text: String = "Press C to toggle godmode : ".to_string()
                        + &(!player.can_take_damage).to_string();
                    let text_pos = Rect::new(10, SCREEN_HEIGHT - 50, 300, 50);
                    let surface = font
                        .render(&immortality_text)
                        .solid(Color::RGBA(0, 255, 0, 255))
                        .map_err(|e| e.to_string())?;
                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .map_err(|e| e.to_string())?;
                    let ui_element = UIElement {
                        texture: texture,
                        position: text_pos,
                    };

                    ui_storage.push(ui_element);
                }
            }
            {
                let gamedatas = game_state.ecs.read_storage::<components::GameData>();
                for gamedata in (gamedatas).join() {
                    {
                        {
                            let score: String = "Score: ".to_string() + &gamedata.score.to_string();
                            let score_text_pos =
                                Rect::new(10 as i32, 0 as i32, 100 as u32, 50 as u32);
                            let surface = font
                                .render(&score)
                                .solid(Color::RGBA(255, 0, 0, 255))
                                .map_err(|e| e.to_string())?;
                            let texture = texture_creator
                                .create_texture_from_surface(&surface)
                                .map_err(|e| e.to_string())?;
                            let ui_element = UIElement {
                                texture: texture,
                                position: score_text_pos,
                            };

                            ui_storage.push(ui_element);
                        }
                        {
                            let level: String = "Level: ".to_string() + &gamedata.level.to_string();
                            let level_text_pos =
                                Rect::new(SCREEN_WIDTH / 2 as i32, 0 as i32, 100 as u32, 50 as u32);
                            let surface = font
                                .render(&level)
                                .solid(Color::RGBA(255, 0, 0, 255))
                                .map_err(|e| e.to_string())?;
                            let texture = texture_creator
                                .create_texture_from_surface(&surface)
                                .map_err(|e| e.to_string())?;
                            let ui_element = UIElement {
                                texture: texture,
                                position: level_text_pos,
                            };
                            ui_storage.push(ui_element);
                        }
                    }
                }
            }

            {
                let render_count_text = "Entity amount: ".to_string()
                    + &game_state.ecs.entities().join().count().to_string();
                let text_pos = Rect::new(SCREEN_WIDTH - 200, 0, 200, 40);
                let surface = font
                    .render(&render_count_text)
                    .solid(Color::RGBA(255, 0, 0, 255))
                    .map_err(|e| e.to_string())?;
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;
                let ui_element = UIElement {
                    texture: texture,
                    position: text_pos,
                };
                ui_storage.push(ui_element);
            }
            {
                let spawn_text: String = "Press V to spawn 20 enemies".to_string();
                let text_pos = Rect::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT - 50, 300, 50);
                let surface = font
                    .render(&spawn_text)
                    .solid(Color::RGBA(0, 255, 0, 255))
                    .map_err(|e| e.to_string())?;
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;
                let ui_element = UIElement {
                    texture: texture,
                    position: text_pos,
                };
                ui_storage.push(ui_element);
            }
        }
        render(
            &mut canvas,
            Color::RGB(0, 0, 0),
            &texture_manager,
            &ui_storage,
            &game_state.ecs,
        )?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
