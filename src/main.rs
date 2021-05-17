extern crate sdl2;
mod player;
//mod ground;
use player::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem.window("Game", 1200, 756).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(135, 206, 235));
    canvas.clear();
    canvas.present();
    let mut event_pump = context.event_pump().unwrap();
    let mut left = false;
    let mut right = false;

    let mut player = Player::new(Rect::new(75, 75, 126, 126));
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

        update(&mut player, left, right);
        draw(&mut canvas, &player);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn update(player: &mut Player, left: bool, right: bool) {
    if (left && right) || (!left && !right) {
        player.input = 0;
    } else if left {
        player.input = -1;
    } else {
        player.input = 1;
    }
    player.update();
}

fn draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, player: &Player) {
    canvas.set_draw_color(Color::RGB(137, 206, 235));
    canvas.clear();
    canvas.set_draw_color(Color::BLACK);
    canvas.fill_rect(player.rect).unwrap();
    canvas.present();
}
