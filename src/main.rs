//! This project is libre, and licenced under the terms of the
//! DO WHAT THE FUCK YOU WANT TO PUBLIC LICENCE, version 3.1,
//! as published by dtf on July 2019. See the COPYING file or
//! https://ph.dtf.wtf/w/wtfpl/#version-3-1 for more details.

extern crate sdl2;
mod player;
mod ground;
mod button;
use player::*;
use button::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    let context = sdl2::init().unwrap();
    let font_context = sdl2::ttf::init().unwrap();
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem.window("Game", 2560, 1440).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(135, 206, 235));
    canvas.clear();
    canvas.present();
    menu(&mut event_pump, &mut canvas);
}

fn menu(event_pump: &mut sdl2::EventPump, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let mut buttons = vec![
        Button::new(Rect::new(16, 16, 2528, 458), Action::Start),
        Button::new(Rect::new(16, 490, 2528, 458), Action::Continue(1)),
        Button::new(Rect::new(16, 964, 2528, 458), Action::Quit)
    ];
    let mut mouse = Mouse::new();
    let level: i32;
    'menu: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => std::process::exit(0),
                Event::KeyDown {keycode, ..} => {
                    match keycode {
                        Some(Keycode::Left) => mouse.left = true,
                        Some(Keycode::Right) => mouse.right = true,
                        Some(Keycode::Up) => mouse.up = true,
                        Some(Keycode::Down) => mouse.down = true,
                        Some(Keycode::Space) => {
                            for button in &buttons {
                                if button.colision {
                                    level = button.function();
                                    break 'menu;
                                }
                            }
                        },
                        _ => (),
                    }
                },
                Event::KeyUp {keycode, ..} => {
                    match keycode {
                        Some(Keycode::Left) => mouse.left = false,
                        Some(Keycode::Right) => mouse.right = false,
                        Some(Keycode::Up) => mouse.up = false,
                        Some(Keycode::Down) => mouse.down = false,
                        _ => (),
                    }
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
        menu_draw(canvas, &mut buttons, &mouse);
        std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }
    game(event_pump, canvas, level);
}

fn game(event_pump: &mut sdl2::EventPump, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, level: i32) {
    let mut left = false;
    let mut right = false;

    let mut map = ground::read(level);

    let mut player = Player::new(Rect::new(64, 64, 64, 64));

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                },
                Event::KeyDown {keycode, ..} => {
                    match keycode {
                        Some(Keycode::Left) => left = true,
                        Some(Keycode::Right) => right = true,
                        Some(Keycode::Space) => player.jump = true,
                        _ => ()
                    }
                },
                Event::KeyUp {keycode, ..} => {
                    match keycode {
                        Some(Keycode::Left) => left = false,
                        Some(Keycode::Right) => right = false,
                        Some(Keycode::Space) => player.jump = false,
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        if let Some(level) = update(&mut player, left, right, &mut map) {
            map = ground::read(level);
        }
        draw(canvas, &player, &map);
        std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }
}

fn update(player: &mut Player, left: bool, right: bool, map: &mut Vec<Vec<ground::Map>>) -> Option<i32> {
    if (left && right) || (!left && !right) {
        player.input = 0;
    } else if left {
        player.input = -1;
    } else {
        player.input = 1;
    }
    player.update(map)
}

fn draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, player: &Player, map: &Vec<Vec<ground::Map>>) {
    canvas.set_draw_color(Color::RGB(137, 206, 235));
    canvas.clear();
    canvas.set_draw_color(Color::BLACK);
    for item in map {
        for tile in item {
            tile.draw(canvas);
        }
    }
    player.draw(canvas);
    canvas.present();
}

fn menu_draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, buttons: &mut Vec::<button::Button>, mouse: &Mouse) {
    canvas.set_draw_color(Color::RGB(137, 206, 235));
    canvas.clear();
    for button in buttons {
        button.draw(canvas);
    }
    mouse.draw(canvas);
    canvas.present();
}
