use std::fs::File;
use std::path::Path;
use sdl2::rect::Rect;

pub fn read(level: i32) -> Vec<Vec<Map>>{
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
                   map[y].push(Map::Ground(Ground::new(x, y as i32)));
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

pub enum Map {
    Air,
    Ground(Ground),
    Goal(Goal),
    Spike(Spike)
}

impl Map {
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        match self {
            Map::Ground(ground) => ground.draw(canvas),
            Map::Goal(goal) => goal.draw(canvas),
            Map::Spike(spike) => spike.draw(canvas),
            Map::Air => (),
        }
    }
}

pub struct Ground {
    pub rect: Rect,
    draw_rect: Rect,
}

impl Ground {
    fn new(x: i32, y: i32) -> Ground {
        Ground {
            rect: Rect::new(x * 16, y * 16, 16, 16),
            draw_rect: Rect::new(x * 64, y * 64, 64, 64),
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
