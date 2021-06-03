use sdl2;

pub enum Action {
    Start,
    Continue(i32),
    Quit,
}

impl Action {
    fn load() -> Action {
        use std::fs::File;
        use std::path::Path;
        use std::io::prelude::*;
        let path_string = "save".to_string();
        let path = Path::new(&path_string);
        let mut file = match File::open(&path) {
            Ok(num) => num,
            _ => return Action::Start,
        };
        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Ok(_) => (),
            Err(_) => return Action::Start,
        };
        match data.parse::<i32>() {
            Ok(num) => return Action::Continue(num),
            Err(_) => return Action::Start,
        }
    }
}

pub struct Button {
    pub rect: sdl2::rect::Rect,
    pub colision: bool,
    pub action: Action,
}

impl Button {
    pub fn new(rect: sdl2::rect::Rect, action_in: Action) -> Button {
        Button {
            rect,
            action: match action_in {
                Action::Quit => Action::Quit,
                Action::Start => Action::Start,
                Action::Continue(_) => Action::load(),
            },
            colision: false,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(32, 32, 32));
        canvas.fill_rect(self.rect).unwrap();
    }
}
