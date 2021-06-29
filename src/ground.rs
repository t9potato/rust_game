use std::fs::File;
use std::path::Path;
use sdl2::rect::Rect;

///Load ground from a level text file t make level edditing easier that with manual definitions in
///the code.
pub fn read(level: i32, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Vec<Vec<Map>>{
    use std::io::prelude::*;
    let pathstr = (format!("maps/level{}.txt", level)).to_string();
    let path = Path::new(&pathstr);
    std::fs::write(Path::new("save"), level.to_string()).unwrap();
    let mut file = File::open(path).expect("Failed to read file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents: Vec<&str> = contents.lines().collect();
    let mut map: Vec<Vec<Map>> = Vec::from(Vec::new());
    let mut x = 0;
    for (y, content) in contents.into_iter().enumerate() {
        map.push(Vec::new());
        for item in content.split(",") {
            match item {
               //"0" => ground::Map::Air,
               "1" => {
                   map[y].push(Map::Ground(Ground::new(x, y as i32, texture_creator)));
                },
                "2" => {
                    map[y].push(Map::Goal(Goal::new(x, y as i32, level + 1)));
                },
                "3" => {
                    map[y].push(Map::Spike(Spike::new(x, y as i32)));
                }
               _ => map[y].push(Map::Air),
            }
            x += 1;
        }
        x = 0;
    }
    map
}

///Usful rust enum to store map data
pub enum Map <'a> {
    Air,
    Ground(Ground<'a>),
    Goal(Goal),
    Spike(Spike)
}

impl <'a> Map <'a> {
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        match self {
            Map::Ground(ground) => ground.draw(canvas),
            Map::Goal(goal) => goal.draw(canvas),
            Map::Spike(spike) => spike.draw(canvas),
            Map::Air => (),
        }
    }
}

pub struct Ground <'a> {
    pub rect: Rect,
    draw_rect: Rect,
    texture: sdl2::render::Texture<'a>
}

impl <'a> Ground <'a> {
    fn new(x: i32, y: i32, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Ground {
        use sdl2::image::LoadTexture;
        Ground {
            rect: Rect::new(x * 16, y * 16, 16, 16),
            draw_rect: Rect::new(x * 64, y * 64, 64, 64),
            texture: texture_creator.load_texture(std::path::Path::new("assets/Tilemap.png")).unwrap(),
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.fill_rect(self.draw_rect).unwrap();
    }
}

pub struct Goal {
    pub rect: Rect,
    draw_rect: Rect,
    pub dest: i32,
}

impl Goal {
    fn new(x: i32, y: i32, dest: i32) -> Goal {
        Goal {
            rect: Rect::new(x * 16, y * 16, 16, 16),
            draw_rect: Rect::new(x * 64, y * 64, 64, 64),
            dest,
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 100, 0));
        canvas.fill_rect(self.draw_rect).unwrap();
    }
}

pub struct Spike {
    pub rect: Rect,
    draw_rect: Rect,
}

impl Spike {
    fn new(x: i32, y: i32) -> Spike {
        Spike {
            rect: Rect::new(x * 16, y * 16, 16, 16),
            draw_rect: Rect::new(x * 64, y * 64, 64, 64),
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(152, 0, 0));
        canvas.fill_rect(self.draw_rect).unwrap();
    }
}
