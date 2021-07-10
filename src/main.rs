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
use std::path::Path;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

fn main() {
    sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();
    let context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = context.video().unwrap();
    sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();
    let window = video_subsystem.window("Game", 2560, 1440).opengl().position_centered().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = context.event_pump().unwrap();
    let mut font = ttf_context.load_font(std::path::Path::new("assets/GnuUnifontFull-Pm9P.ttf"), 32).unwrap();
    let texture_creator = canvas.texture_creator();

    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

    let sound_chunk = sdl2::mixer::Chunk::from_file(Path::new("assets/BeepBox-Song.wav")).unwrap();
    sdl2::mixer::Channel::all().play(&sound_chunk, -1).unwrap();

    canvas.set_draw_color(Color::RGB(135, 206, 235));
    canvas.clear();
    canvas.present();
    menu(&mut event_pump, &mut canvas, &mut font, &texture_creator);
}

fn menu(event_pump: &mut sdl2::EventPump, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, font: &mut sdl2::ttf::Font, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) {
    let mut buttons = vec![
        Button::new(Rect::new(1158, 990, 512, 64), Action::Start),
        Button::new(Rect::new(1158, 1086, 512, 64), Action::Continue(||1)),
        Button::new(Rect::new(1158, 1182, 512, 64), Action::ClearSave),
        Button::new(Rect::new(1158, 1278, 512, 64), Action::Quit),
    ];
    let mut mouse = Mouse::new(texture_creator);
    let texture = texture_creator.load_texture(std::path::Path::new("assets/title.png")).unwrap();
    let mut level = 0;
    loop {
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
                                level = button.function()
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

        menu_draw(canvas, &mut buttons, &mouse, font, &texture_creator, &texture);
        if level != 0 {
            game(event_pump, canvas, level, texture_creator, font);
            level = 0;
        }

        std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }
}

fn game(event_pump: &mut sdl2::EventPump, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        level: i32, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>, font: &mut sdl2::ttf::Font) {
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
    player.sounds[1].set_volume(64);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } =>  std::process::exit(0),
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Left) => left = true,
                    Some(Keycode::Right) => right = true,
                    Some(Keycode::LShift) | Some(Keycode::RShift) => player.jump = true,
                    Some(Keycode::Escape) => break 'main,
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

        draw(canvas, &mut player, &mut map, &map_textures, texture_creator, font);

        if let Some(level) = update(&mut player, left, right, &mut map, canvas.output_size().unwrap()) {
            enter_door(canvas, &mut map, &map_textures, texture_creator);
            map = ground::read(level);
        }
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

fn draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, player: &mut Player, map: &mut Vec<Vec<ground::Map>>,
        map_textures: &[sdl2::render::Texture], texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font: &mut sdl2::ttf::Font) {
    canvas.set_draw_color(Color::RGB(141, 183, 255));
    canvas.clear();
    canvas.set_draw_color(Color::BLACK);
    let mut x = 0;
    let mut pos: Vec<Vec2> = Vec::new();
    for (y, item) in (*map).iter_mut().enumerate() {
        for tile in item {
            if tile.draw(canvas, &map_textures) {
                pos.push(Vec2(x, y as i32))
            }
            x += 1;
        }
        x = 0;
    }
    player.draw(canvas, font, texture_creator);
    
    let circle = gfx::light::circle(32, Color::RGB(15, 15, 15), texture_creator);
    for item in pos {
        if let ground::Map::Torch(torch) = &map[item.1 as usize][item.0 as usize] {
            canvas.copy(&circle, None, sdl2::rect::Rect::new(torch.pos.x - 92, torch.pos.y - 120, 256, 256)).unwrap();
            canvas.copy(&circle, None, sdl2::rect::Rect::new(torch.pos.x - 28, torch.pos.y - 56, 128, 128)).unwrap();
            canvas.copy(&circle, None, sdl2::rect::Rect::new(torch.pos.x + 5, torch.pos.y - 26, 64, 64)).unwrap();
            canvas.copy(&circle, None, sdl2::rect::Rect::new(torch.pos.x + 21, torch.pos.y - 10, 32, 32)).unwrap();
        }
    }
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

fn enter_door(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, map: &mut Vec<Vec<ground::Map>>, map_textures: &[sdl2::render::Texture], texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) {
    let mut rect: Option<Rect> = None;
    for item in (*map).iter_mut() {
        for tile in item {
            if let ground::Map::Goal(pos) = tile {
                if  rect.is_none() {
                    rect = Some(Rect::new(pos.draw_rect.x - 64, pos.draw_rect.y, 128, 192));
                }
            }
        }
    }
    let rect = rect.unwrap();
    let anim_texture = texture_creator.load_texture(std::path::Path::new("assets/door_enter.png")).unwrap();

    for i in 0..13 {
        canvas.set_draw_color(Color::RGB(141, 183, 255));
        canvas.clear();
        canvas.set_draw_color(Color::BLACK);
        let mut x = 0;
        let mut pos: Vec<Vec2> = Vec::new();
        for (y, item) in (*map).iter_mut().enumerate() {
            for tile in item {
                if tile.draw(canvas, &map_textures) {
                    pos.push(Vec2(x, y as i32))
                }
                x += 1;
            }
            x = 0;
        }
        let circle = gfx::light::circle(32, sdl2::pixels::Color::RGB(15, 15, 15), texture_creator);
        canvas.copy(&anim_texture, Rect::new(32 * i, 0, 32, 48), rect).unwrap();
        for item in pos {
            if let ground::Map::Torch(torch) = &map[item.1 as usize][item.0 as usize] {
                canvas.copy(&circle, None, sdl2::rect::Rect::new(torch.pos.x - 92, torch.pos.y - 120, 256, 256)).unwrap();
                canvas.copy(&circle, None, sdl2::rect::Rect::new(torch.pos.x - 28, torch.pos.y - 56, 128, 128)).unwrap();
                canvas.copy(&circle, None, sdl2::rect::Rect::new(torch.pos.x + 5, torch.pos.y - 26, 64, 64)).unwrap();
                canvas.copy(&circle, None, sdl2::rect::Rect::new(torch.pos.x + 21, torch.pos.y - 10, 32, 32)).unwrap();
            }
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1000000000u32 / 25));
    }
}
