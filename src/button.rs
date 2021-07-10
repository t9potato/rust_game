pub enum Action {
    Start,
    Continue(fn()->i32),
    ClearSave,
    Quit,
}

impl Action {
    fn load() -> Action {
        Action::Continue(||{
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;
        let path_string = "save".to_string();
        let path = Path::new(&path_string);
        let mut file = match File::open(&path) {
            Ok(num) => num,
            _ => return 1,
        };
        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Ok(_) => (),
            Err(_) => return 1,
        };

        match data.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => 1,
        }
        })
    }
}

pub struct Button {
    pub rect: sdl2::rect::Rect,
    draw_rect: sdl2::rect::Rect,
    pub colision: bool,
    pub action: Action,
}

impl Button {
    pub fn new(rect: sdl2::rect::Rect, action_in: Action) -> Button {
        Button {
            rect,
            draw_rect: sdl2::rect::Rect::new(
                rect.x + 4,
                rect.y + 4,
                rect.width() - 8,
                rect.height() - 8,
            ),
            action: match action_in {
                Action::Quit => Action::Quit,
                Action::Start => Action::Start,
                Action::ClearSave => Action::ClearSave,
                Action::Continue(_) => Action::load(),
            },
            colision: false,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, font: &mut sdl2::ttf::Font, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) {
        use sdl2::rect::Rect;
        if self.colision {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(217, 189, 200));
        } else {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        }
        canvas.fill_rect(self.draw_rect).unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(31, 16, 42));
        canvas.fill_rect(Rect::new(self.rect.x + 4, self.rect.y, self.rect.width() - 8, 4)).unwrap();
        canvas.fill_rect(Rect::new( self.rect.x + 4, self.rect.y + self.rect.height() as i32 - 4, self.rect.width() - 8, 4)).unwrap();
        canvas.fill_rect(Rect::new(self.rect.x, self.rect.y + 4, 4, self.rect.height() - 8)).unwrap();
        canvas.fill_rect(Rect::new(self.rect.x + self.rect.width() as i32 - 4, self.rect.y + 4, 4, self.rect.height() - 8)).unwrap();
        let text_color = sdl2::pixels::Color::RGB(31, 16, 42);
        match self.action {
            Action::Quit => {
                let surface = font.render("QUIT").blended(text_color).unwrap();
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .unwrap();
                canvas.copy(&texture, None, Some(Rect::new(self.rect.center().x - 64, self.rect.y, 128, self.rect.height()))).unwrap();
            }
            Action::Continue(..) => {
                let surface = font.render("CONTINUE").blended(text_color).unwrap();
                let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                canvas.copy(&texture, None, Some(Rect::new(self.rect.center().x - 128, self.rect.y, 256, self.rect.height()))).unwrap();
            }
            Action::Start => {
                let surface = font.render("START").blended(text_color).unwrap();
                let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                canvas.copy(&texture, None, Some(Rect::new(self.rect.center().x - 80, self.rect.y, 160, self.rect.height()))).unwrap();
            }
            Action::ClearSave => {
                let surface = font.render("CLEAR SAVE").blended(text_color).unwrap();
                let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                canvas.copy(&texture, None, Some(Rect::new(self.rect.center().x - 160, self.rect.y, 320, 64))).unwrap();
            }
        }
    }

    pub fn function(&self) -> i32 {
        match self.action {
            Action::Quit => std::process::exit(0),
            Action::Start => 1,
            Action::Continue(num) => num(),
            Action::ClearSave => 0,
        }
    }
}

///A basic character to replicate the mouse so it can be used with a joystick
pub struct Mouse <'a> {
    pub rect: sdl2::rect::Rect,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    texture: sdl2::render::Texture<'a>,
}

impl <'a> Mouse <'a> {
    pub fn new(texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Mouse {
        use sdl2::image::LoadTexture;
        Mouse {
            rect: sdl2::rect::Rect::new(1264, 704, 32, 32),
            up: false,
            down: false,
            left: false,
            right: false,
            texture: texture_creator.load_texture("assets/cursor.png").unwrap(),
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
            canvas.copy(&self.texture, None, self.rect).unwrap();
    }
}
