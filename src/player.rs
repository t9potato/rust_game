//!The code for the player character

use crate::ground::*;
use sdl2::rect::Rect;
use crate::gfx;
use rand::Rng;
use std::cmp::Ordering;

pub struct Vec2(pub i32, pub i32);

///This class hase some spageti, but it is still decently readable
pub struct Player<'a> {
    pub rect: Rect,
    start_pos: Vec2,
    previous_position: Rect,
    draw_rect: Rect,
    vel: Vec2,
    min_vel: Vec2,
    max_vel: Vec2,
    pub input: i8,
    pub jump: bool,
    grounded: bool,
    pub death_count: u16,
    death_texture: sdl2::render::Texture<'a>,
    texture: sdl2::render::Texture<'a>,
    animation_num: i32,
    particles: Vec<gfx::particles::Full>,
    particle_delay: i32,
    sounds: Vec<sdl2::mixer::Chunk>,
}

impl<'a> Player<'a> {
    pub fn new(rect: Rect, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Player {
        use sdl2::image::LoadTexture;
        Player {
            draw_rect: Rect::new(rect.x, rect.y + 1, rect.width(), rect.height()),
            rect: Rect::new(rect.x / 4, rect.y / 4, rect.width() / 4, rect.height() / 4),
            start_pos: Vec2(rect.x / 4, rect.y / 4),
            previous_position: Rect::new(
                rect.x / 4,
                rect.y / 4,
                rect.width() / 4,
                rect.height() / 4,
            ),
            vel: Vec2(0, 0),
            min_vel: Vec2(-2, -8),
            max_vel: Vec2(2, 8),
            input: 0,
            jump: false,
            grounded: false,
            death_count: 0,
            death_texture: texture_creator.load_texture("assets/death.png").unwrap(),
            texture: texture_creator.load_texture("assets/Player.png").unwrap(),
            animation_num: 0,
            particles: Vec::new(),
            particle_delay: 0,
            sounds: vec![
                sdl2::mixer::Chunk::from_file(std::path::Path::new("assets/die.wav")).unwrap(),
                sdl2::mixer::Chunk::from_file(std::path::Path::new("assets/jump.wav")).unwrap(),
            ],
        }
    }

    pub fn update(&mut self, map: &mut Vec<Vec<Map>>, canvas_size: (u32, u32)) -> Option<i32> {
        match self.input {
            1 => {
                self.vel.0 += 1;
            }
            0 => {
                match self.vel.0.cmp(&0) {
                    Ordering::Greater => self.vel.0 -= 1,
                    Ordering::Less => self.vel.0 += 1,
                    Ordering::Equal => ()
                }
            }
            -1 => {
                self.vel.0 -= 1;
            }
            _ => (),
        }

        self.vel = clamp(&self.vel, &self.min_vel, &self.max_vel);
        let ground_num = self.mov_pos(map, canvas_size);

        if ground_num > 1 {
            return Some(ground_num - 1);
        } else if ground_num == -1 {
            (0..15).into_iter().for_each(|_| (
                self.particles.push(gfx::particles::Full::new((self.draw_rect.x + 32) as f32, (self.draw_rect.y + 32) as f32, rand::thread_rng().gen_range(0.0..1.0) * -self.vel.0 as f32,
                rand::thread_rng().gen_range(0.0..1.0) * self.vel.1 as f32, rand::thread_rng().gen_range(1.0..3.0), sdl2::pixels::Color::BLACK))
            ));
            sdl2::mixer::Channel::all().play(&self.sounds[0], 0).unwrap();
            self.vel = Vec2(0, 0);
            self.rect.x = self.start_pos.0;
            self.rect.y = self.start_pos.1;
            self.death_count += 1;
        }
        if self.jump && self.grounded {
            self.jump();
        } else if !self.grounded {
            self.fall();
        } else if self.grounded {
            self.vel.1 = 0;
        }
        self.draw_rect.x = self.rect.x * 4;
        self.draw_rect.y = self.rect.y * 4;
        self.previous_position.x = self.rect.x;
        self.previous_position.y = self.rect.y - 1;

        let mut index = 0;
        let mut indexes: Vec<usize> = Vec::new();
        for particle in &mut self.particles {
            if particle.update() {
                indexes.push(index);
            }
            index += 1;
        }
        index = 0;
        for item in indexes {
            self.particles.remove(item - index);
            index += 1;
        }
        if self.particle_delay != 0 {
            self.particle_delay -= 1;
        }

        self.grounded = false;

        None
    }

    fn mov_pos(&mut self, map: &mut Vec<Vec<Map>>, canvas_size: (u32, u32)) -> i32 {
        let mut return_val = 0;
        self.rect.x += self.vel.0;
        let mut collisions: Vec<&Map> = Vec::new();

        for row in &*map {
            for tile in row {
                match tile {
                    Map::Goal(goal) => {
                        if let Some(..) = self.rect.intersection(goal.rect) {
                            self.rect.x = self.start_pos.0;
                            self.rect.y = self.start_pos.1;
                            return 1 + goal.dest;
                        }
                    },
                    Map::Ground(Ground { rect, .. }) | Map::Spike(Spike { rect, .. }) => {
                        if let Some(..) = self.rect.intersection(*rect) {
                            collisions.push(tile);
                        }
                    }, 
                    _ => (),
                }
            }
        }

        for tile in collisions {
            match self.vel.0.cmp(&0) {
                Ordering::Greater => {
                    match tile {
                        Map::Ground(ground) => self.rect.x = ground.rect.x - self.rect.w,
                        Map::Spike(spike) => if let Some(..) = self.rect.intersection(spike.rect) {
                            return_val = -1;
                        },
                        _ => (),
                    };
                },
                Ordering::Less => {
                    match tile {
                        Map::Ground(ground) => self.rect.x = ground.rect.w + ground.rect.x,
                        Map::Spike(spike) => if let Some(..) = self.rect.intersection(spike.rect) {
                            return_val = -1;
                        },
                        _ => (),
                    };
                },
                _ => (),
            }
        };

        if return_val != 0 {
            return return_val
        }

        let mut collisions: Vec<&Map> = Vec::new();
        self.rect.y += self.vel.1;
        if self.draw_rect.y >= canvas_size.1 as i32 {
            return -1
        }


        for row in &*map {
            for tile in row {
                match tile {
                    Map::Goal(goal) => {
                        if let Some(..) = self.rect.intersection(goal.rect) {
                            self.rect.x = self.start_pos.0;
                            self.rect.y = self.start_pos.1;
                            return 1 + goal.dest;
                        }
                    },
                    Map::Ground(Ground { rect, .. }) | Map::Spike(Spike { rect, .. }) => {
                        if let Some(..) = self.rect.intersection(*rect) {
                            collisions.push(tile);
                        }
                    },
                    _ => (),
                }
            }
        }

        for tile in collisions {
            match self.vel.1.cmp(&0) {
                Ordering::Greater => {
                    match tile {
                        Map::Ground(ground) => {
                            self.grounded = true;
                            self.rect.y = ground.rect.y - ground.rect.h;
                            return_val = 1;
                        },
                        Map::Spike(spike) => if let Some(..) = self.rect.intersection(spike.rect) {
                            return_val = -1;
                        },
                        _ => (),
                    };
                },
                Ordering::Less => {
                    match tile {
                        Map::Ground(ground) => self.rect.y = ground.rect.y + self.rect.h,
                        Map::Spike(spike) => if let Some(..) = self.rect.intersection(spike.rect) {
                            return_val = -1;
                        },
                        _ => (),
                    };
                },
                _ => (),
            };
        }

        return_val
    }


    fn jump(&mut self) {
        self.vel.1 = -8;
        if self.particle_delay == 0 {
            let num = rand::thread_rng().gen_range(10..20);
            (0..num).into_iter().for_each(|_| {
                self.particles.push(gfx::particles::Full::new (
                        (self.draw_rect.x + 32) as f32,
                        (self.draw_rect.y + 64) as f32,
                        rand::thread_rng().gen_range(-2.0..=2.0),
                        rand::thread_rng().gen_range(-2.0..=2.0),
                        rand::thread_rng().gen_range(1.9..2.5),
                        match rand::thread_rng().gen_range(0..4) {
                            0 => sdl2::pixels::Color::RGB(204, 66, 94),
                            1 => sdl2::pixels::Color::RGB(163, 40, 88),
                            2 => sdl2::pixels::Color::RGB(107, 201, 108),
                            _ => sdl2::pixels::Color::RGB(91, 166, 117),
                        },
                        ));
            });
            self.particle_delay = 10;
            sdl2::mixer::Channel::all().play(&self.sounds[1], 0).unwrap();
        }
    }

    fn fall(&mut self) {
        self.vel.1 += 1;
    }

    pub fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, font: &mut sdl2::ttf::Font, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) {
        self.animation_num += 1;
        if self.animation_num == 54 {
            self.animation_num = 0;
        }
        if self.animation_num < 49 {
            canvas.copy(&self.texture, Rect::new(0, 0, 16, 16), self.draw_rect).unwrap();
        } else {
            canvas.copy(&self.texture, Rect::new(16, 0, 16, 16), self.draw_rect).unwrap();
        }

        for particle in &self.particles {
            particle.draw(canvas);
        }

        let surface = font.render(format!(":{}", self.death_count).as_str()).blended(sdl2::pixels::Color::RGB(31, 16, 42)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        canvas.copy(&self.death_texture, None, Some(Rect::new(16, 16, 64, 64))).unwrap();
        canvas.copy(&texture, None, Some(Rect::new(80, 16, format!(":{}", self.death_count).len() as u32 * 64, 64))).unwrap();

    }
}

///Function used to make there numbers in 1 vector fit inside the range of 2 other vectors by
///trimming
fn clamp(num: &Vec2, min: &Vec2, max: &Vec2) -> Vec2 {
    let mut ans = Vec2(num.0, num.1);
    if num.0 < min.0 {
        ans.0 = min.0
    } else if num.0 > max.0 {
        ans.0 = max.0
    }
    if num.1 < min.1 {
    } else if num.1 > max.1 {
        ans.1 = max.1
    }
    ans
}
