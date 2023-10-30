use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use specs::{DispatcherBuilder, Join, World, WorldExt};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

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

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture_creator: &TextureCreator<WindowContext>,
    texture_manager: &texture_manager::TextureManager,
    font: &sdl2::ttf::Font,
    ecs: &World,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let positions = ecs.read_storage::<components::Position>();
    let mut renderables = ecs.write_storage::<components::Renderable>();
    let players = ecs.read_storage::<components::Player>();

    {
        let render_count_text = "Entity amount: ".to_string() + &renderables.count().to_string();
        let text_position = Rect::new(SCREEN_WIDTH - 200, 0, 100, 20);
        render_text(
            &render_count_text,
            text_position,
            Color::RGBA(0, 255, 0, 255),
            texture_creator,
            font,
            canvas,
        )?;
    }
    for (renderable, position) in (&mut renderables, &positions).join() {
        let screen_rect = Rect::from_center(
            position.pos,
            renderable.output_width,
            renderable.output_height,
        );
        // let texture = texture_creator.load_texture(&renderable.texture_name)?;
        let texture = texture_manager
            .get_texture(&renderable.texture_name)
            .unwrap();
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

    let gamedatas = ecs.read_storage::<components::GameData>();
    for gamedata in (gamedatas).join() {
        {
            let score: String = "Score: ".to_string() + &gamedata.score.to_string();
            let score_text_pos = Rect::new(10 as i32, 0 as i32, 100 as u32, 50 as u32);
            render_text(
                &score,
                score_text_pos,
                Color::RGBA(255, 0, 0, 255),
                texture_creator,
                font,
                canvas,
            )?;

            let level: String = "Level: ".to_string() + &gamedata.level.to_string();
            let level_text_pos =
                Rect::new(SCREEN_WIDTH / 2 as i32, 0 as i32, 100 as u32, 50 as u32);
            render_text(
                &level,
                level_text_pos,
                Color::RGBA(255, 0, 0, 255),
                texture_creator,
                font,
                canvas,
            )?;
        }
    }
    for player in (players).join() {
        let immortality_text: String =
            "Press C to toggle godmode : ".to_string() + &(!player.can_take_damage).to_string();
        let text_pos = Rect::new(10, SCREEN_HEIGHT - 50, 300, 50);
        render_text(
            &immortality_text,
            text_pos,
            Color::RGBA(0, 255, 0, 255),
            texture_creator,
            font,
            canvas,
        )?;
        {
            let spawn_text: String = "Press V to spawn 20 enemies".to_string();
            let text_pos = Rect::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT - 50, 300, 50);
            render_text(
                &spawn_text,
                text_pos,
                Color::RGBA(0, 255, 0, 255),
                texture_creator,
                font,
                canvas,
            )?;
        }
    }

    canvas.present();
    Ok(())
}

fn render_text(
    text: &String,
    text_position: Rect,
    color: Color,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    canvas: &mut WindowCanvas,
) -> Result<(), String> {
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    canvas.copy(&texture, None, Some(text_position))?;
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
    texture_manager.load_texture(
        &String::from("marco"),
        &String::from("assets/marco.png"),
        &texture_creator,
    )?;
    texture_manager.load_texture(
        &String::from("enemy"),
        &String::from("assets/running.png"),
        &texture_creator,
    )?;
    texture_manager.load_texture(
        &String::from("rocket"),
        &String::from("assets/rocket.png"),
        &texture_creator,
    )?;

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

    let mut dispacher = DispatcherBuilder::new()
        .with(asteroid::AsteroidMover, "asteroid_mover", &[])
        .with(asteroid::AstroidCollider, "asteroid_collider", &[])
        .with(rocket::RocketMover, "rocket_mover", &[])
        .with(rocket::RocketDamage, "rocket_damage", &[])
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
            &texture_manager,
            &font,
            &game_state.ecs,
        )?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
