extern crate sdl2;
mod button;
mod ground;
mod gfx;
mod player;
use button::*;
use player::*;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
    let context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem
        .window("Game", 2560, 1440)
        .opengl()
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = context.event_pump().unwrap();
    let mut font = ttf_context
        .load_font(std::path::Path::new("assets/GnuUnifontFull-Pm9P.ttf"), 32)
        .unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(135, 206, 235));
    canvas.clear();
    canvas.present();
    menu(&mut event_pump, &mut canvas, &mut font, &texture_creator);
}

fn menu(
    event_pump: &mut sdl2::EventPump,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &mut sdl2::ttf::Font,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) {
    let mut buttons = vec![
        Button::new(Rect::new(1158, 1088, 512, 64), Action::Start),
        Button::new(Rect::new(1158, 1182, 512, 64), Action::Continue(1)),
        Button::new(Rect::new(1158, 1280, 512, 64), Action::Quit),
    ];
    let mut mouse = Mouse::new(texture_creator);
    let level: i32;
    let texture = texture_creator
        .load_texture(std::path::Path::new("assets/title.png"))
        .unwrap();
    'menu: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Left) => mouse.left = true,
                    Some(Keycode::Right) => mouse.right = true,
                    Some(Keycode::Up) => mouse.up = true,
                    Some(Keycode::Down) => mouse.down = true,
                    Some(Keycode::LShift) | Some(Keycode::RShift) => {
                        for button in &buttons {
                            if button.colision {
                                level = button.function();
                                break 'menu;
                            }
                        }
                    }
                    _ => (),
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::Left) => mouse.left = false,
                    Some(Keycode::Right) => mouse.right = false,
                    Some(Keycode::Up) => mouse.up = false,
                    Some(Keycode::Down) => mouse.down = false,
                    _ => (),
                },
                _ => (),
            }
        }
        mouse.update();
        for button in &mut buttons {
            match button.rect.intersection(mouse.rect) {
                Some(_) => button.colision = true,
                None => button.colision = false,
            }
        }
        menu_draw(
            canvas,
            &mut buttons,
            &mouse,
            font,
            &texture_creator,
            &texture,
        );
        std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }
    game(event_pump, canvas, level, texture_creator);
}

fn game(
    event_pump: &mut sdl2::EventPump,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    level: i32,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) {
    let mut left = false;
    let mut right = false;

    let map_textures = vec![
        texture_creator.load_texture(std::path::Path::new("assets/Tilemap.png")).unwrap(),
        texture_creator.load_texture(std::path::Path::new("assets/enemy.png")).unwrap(),
        texture_creator.load_texture(std::path::Path::new("assets/door.png")).unwrap(),
        texture_creator.load_texture(std::path::Path::new("assets/torch.png")).unwrap(),
    ];
    let mut map = ground::read(level);
    let mut player = Player::new(Rect::new(64, 1312, 64, 64), texture_creator);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'main;
                }
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Left) => left = true,
                    Some(Keycode::Right) => right = true,
                    Some(Keycode::LShift) | Some(Keycode::RShift) => player.jump = true,
                    _ => (),
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::Left) => left = false,
                    Some(Keycode::Right) => right = false,
                    Some(Keycode::LShift) | Some(Keycode::RShift) => player.jump = false,
                    _ => (),
                },
                _ => (),
            }
        }

        if let Some(level) = update(&mut player, left, right, &mut map, canvas.output_size().unwrap()) {
            map = ground::read(level);
        }
        draw(canvas, &mut player, &mut map, &map_textures);
        std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }
}

fn update(player: &mut Player, left: bool, right: bool, map: &mut Vec<Vec<ground::Map>>, canvas_size: (u32, u32)) -> Option<i32> {
    if (left && right) || (!left && !right) {
        player.input = 0;
    } else if left {
        player.input = -1;
    } else {
        player.input = 1;
    }
    player.update(map, canvas_size)
}

fn draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, player: &mut Player, map: &mut Vec<Vec<ground::Map>>, map_textures: &Vec<sdl2::render::Texture>) {
    canvas.set_draw_color(Color::RGB(141, 183, 255));
    canvas.clear();
    canvas.set_draw_color(Color::BLACK);
    for item in map {
        for tile in item {
            tile.draw(canvas, &map_textures);
        }
    }
    player.draw(canvas);
    canvas.present();
}

fn menu_draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, buttons: &mut Vec<button::Button>, mouse: &Mouse, font: &mut sdl2::ttf::Font, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>, texture: &sdl2::render::Texture) {
    canvas.set_draw_color(Color::RGB(141, 183, 255));
    canvas.clear();
    canvas.copy(texture, None, None).unwrap();
    for button in buttons {
        button.draw(canvas, font, texture_creator);
    }
    mouse.draw(canvas);
    canvas.present();
}
