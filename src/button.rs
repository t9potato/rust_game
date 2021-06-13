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
        //find error
        //println!("{}", data);
        match data.trim().parse::<i32>() {
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

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, font: &mut sdl2::ttf::Font, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) {
        use sdl2::rect::Rect;
        if self.colision {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(32, 32, 32));
        } else {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(132, 132, 132));
        }
        canvas.fill_rect(self.rect).unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        match self.action {
            Action::Quit => {
                let surface = font.render("QUIT").blended(sdl2::pixels::Color::BLACK).unwrap();
                let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                canvas.copy(&texture, None, Some(Rect::new(self.rect.center().x - 64, self.rect.y, 128, self.rect.height()))).unwrap();
            },
            Action::Continue(..) => {
                let surface = font.render("CONTINUE").blended(sdl2::pixels::Color::BLACK).unwrap();
                let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                canvas.copy(&texture, None, Some(Rect::new(self.rect.center().x - 128, self.rect.y, 256, self.rect.height()))).unwrap();
            },
            Action::Start => {
                let surface = font.render("START").blended(sdl2::pixels::Color::BLACK).unwrap();
                let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                canvas.copy(&texture, None, Some(Rect::new(self.rect.center().x - 80, self.rect.y, 160, self.rect.height()))).unwrap();
            },
        }
    }

    pub fn function(&self) -> i32 {
        match self.action {
            Action::Quit => std::process::exit(0),
            Action::Start => 1,
            Action::Continue(num) => num,
        }
    }
}

pub struct Mouse {
    pub rect: sdl2::rect::Rect,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            rect: sdl2::rect::Rect::new(1264, 704, 32, 32),
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn update(&mut self) {
        if self.up {
            self.rect.y -= 6;
        }
        if self.down {
            self.rect.y += 6;
        }
        if self.left {
            self.rect.x -= 6;
        }
        if self.right {
            self.rect.x += 6;
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(152, 0, 0));
        canvas.fill_rect(self.rect).unwrap();
    }
}
