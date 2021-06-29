use std::fs::File;
use std::path::Path;
use sdl2::rect::Rect;

///Load ground from a level text file t make level edditing easier that with manual definitions in
///the code.
pub fn read(level: i32) -> Vec<Vec<Map>>{
    use std::io::prelude::*;
    let pathstr = (format!("maps/level{}.txt", level)).to_string();
    let path = Path::new(&pathstr);
    std::fs::write(Path::new("save"), level.to_string()).unwrap();

    let mut file = File::open(path).expect("Failed to read file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut content: Vec<Vec<&str>> = Vec::from(Vec::new());
    let mut map: Vec<Vec<Map>> = Vec::from(Vec::new());

    let mut y = 0;
    for line in contents.lines().into_iter() {
        content.push(Vec::new());
        for item in line.split(',') {
            content[y].push(item);
        }
        y += 1;
    }

    let x_len = content[1].len() - 1;
    let y_len = content.len() - 1;
    let mut x = 0;
    y = 0;
    for line in &content {
        map.push(Vec::new());
        for item in line {
            match item {
               //"0" => ground::Map::Air,
               &"1" => {
                   let mut args = (false, false, false, false);
                   if y != 0 && &content[y-1][x] == &"1" {
                       args.0 = true;
                   }
                   if y != y_len && content[y+1][x] == "1" {
                       args.1 = true;
                   }
                   if x != 0 && content[y][x-1] == "1" {
                       args.2 = true;
                   }
                   if x != x_len && content[y][x+1] == "1" {
                       args.3 = true;
                   }
                   map[y].push(Map::Ground(Ground::new(x as i32, y as i32, args)));
                },
                &"2" => {
                    let pos: bool;
                    if y == 0 || content[y-1][x] != "2" {
                        pos = true;
                    } else {
                        pos = false;
                    }
                    map[y].push(Map::Goal(Goal::new(x as i32, y as i32, level + 1, pos)));
                },
                &"3" => {
                    map[y].push(Map::Spike(Spike::new(x as i32, y as i32)));
                }
               _ => map[y].push(Map::Air),
            }
            x += 1;
        }
        x = 0;
        y += 1
    }
    map
}

///Usful rust enum to store map data
pub enum Map {
    Air,
    Ground(Ground),
    Goal(Goal),
    Spike(Spike)
}

impl Map {
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, map_textures: &Vec<sdl2::render::Texture>) {
        match self {
            Map::Ground(ground) => ground.draw(canvas, &map_textures[0]),
            Map::Goal(goal) => goal.draw(canvas, &map_textures[2]),
            Map::Spike(spike) => spike.draw(canvas, &map_textures[1]),
            Map::Air => (),
        }
    }
}

pub struct Ground {
    pub rect: Rect,
    draw_rect: Rect,
    image_rect: Rect,
}

impl Ground {
    fn new(x: i32, y: i32, sides: (bool, bool, bool, bool)) -> Ground {
        //above, below left right
        let image_rect = match sides {
            //singe tile
            (false, false, false, false) => Rect::new(0,0,16,16),
            //botom single stack
            (true, false, false, false) => Rect::new(192,0,16,16),
            //botom right
            (true, false, true, false) => Rect::new(240,0,16,16),
            //botom left
            (true, false, false, true) => Rect::new(208,0,16,16),
            //botom middle
            (true, false, true, true) => Rect::new(224,0,16,16),
            //midle single stack
            (true, true, false, false) => Rect::new(128,0,16,16),
            //midle right
            (true, true, true, false) => Rect::new(176,0,16,16),
            //midle left
            (true, true, false, true) => Rect::new(144,0,16,16),
            //midle center
            (true, true, true, true) => Rect::new(160,0,16,16),
            //top single stack
            (false, true, false, false) => Rect::new(64,0,16,16),
            //top right
            (false, true, true, false) => Rect::new(112,0,16,16),
            //top left
            (false, true, false, true) => Rect::new(80,0,16,16),
            //top midle
            (false, true, true, true) => Rect::new(96,0,16,16),
            //top long single stack right
            (false, false, true, false) => Rect::new(48,0,16,16),
            //top long single stack midle
            (false, false, true, true) => Rect::new(32,0,16,16),
            //top long single stack left
            (false, false, false, true) => Rect::new(16,0,16,16),
        };
        Ground {
            rect: Rect::new(x * 16, y * 16, 16, 16),
            draw_rect: Rect::new(x * 64, y * 64, 64, 64),
            image_rect,
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture: &sdl2::render::Texture) {
        canvas.copy(texture, self.image_rect, self.draw_rect).unwrap();
    }
}

pub struct Goal {
    pub rect: Rect,
    draw_rect: Rect,
    texture_rect: Rect,
    pub dest: i32,
}

impl Goal {
    fn new(x: i32, y: i32, dest: i32, top: bool) -> Goal {
        Goal {
            rect: Rect::new(x * 16, y * 16, 16, 16),
            draw_rect: Rect::new(x * 64, y * 64, 64, 64),
            texture_rect: match top {
                true => Rect::new(0, 0, 16, 16),
                false => Rect::new(16, 0, 16, 16),
            },
            dest,
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture: &sdl2::render::Texture) {
        canvas.copy(texture, self.texture_rect, self.draw_rect).unwrap();
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

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture: &sdl2::render::Texture) {
        canvas.copy(texture, None, self.draw_rect).unwrap()
    }
}
