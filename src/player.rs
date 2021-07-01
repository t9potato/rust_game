//!The code for the player character

use crate::ground::*;
use sdl2::rect::Rect;

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
    pub death_count: i8,
    grounded: bool,
    texture: sdl2::render::Texture<'a>,
    animation_num: i32,
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
            texture: texture_creator.load_texture("assets/Player.png").unwrap(),
            animation_num: 0,
        }
    }

    pub fn update(&mut self, map: &mut Vec<Vec<Map>>, canvas_size: (u32, u32)) -> Option<i32> {
        match self.input {
            1 => {
                self.vel.0 += 1;
            }
            0 => {
                if self.vel.0 < 0 {
                    self.vel.0 += 1;
                } else if self.vel.0 > 0 {
                    self.vel.0 -= 1;
                }
            }
            -1 => {
                self.vel.0 -= 1;
            }
            _ => (),
        }

        self.vel = clamp(&self.vel, &self.min_vel, &self.max_vel);
        self.mov_pos();
        let ground_num = self.grounded(map, canvas_size);
        if ground_num == 0 {
            self.grounded = false;
        } else if ground_num == 1 {
            self.grounded = true;
        } else if ground_num > 1 {
            return Some(ground_num - 1);
        } else if ground_num == -1 {
            self.vel = Vec2(0, 0);
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
        self.draw_rect.y = (self.rect.y - 1) * 4;
        self.previous_position.x = self.rect.x;
        self.previous_position.y = self.rect.y - 1;
        None
    }

    fn mov_pos(&mut self) {
        self.rect.x += self.vel.0;
        self.rect.y += self.vel.1;
    }

    fn jump(&mut self) {
        self.vel.1 = -8;
    }

    fn grounded(&mut self, tiles: &mut Vec<Vec<Map>>, canvas_size: (u32, u32)) -> i32 {
        let mut return_num = 0;
        if self.rect.y > (canvas_size.1 / 4) as i32 {
            self.rect.x = self.start_pos.0;
            self.rect.y = self.start_pos.1;
            return -1;
        }
        'map: for rows in tiles {
            for tile in rows {
                match tile {
                    Map::Ground(floor) => {
                        if let Some(..) = self.rect.intersection(floor.rect) {
                            match self.ajust_pos(floor.rect) {
                                1 => return_num = 1,
                                _ => (),
                            }
                        }
                    }
                    Map::Goal(goal) => {
                        if let Some(..) = self.rect.intersection(goal.rect) {
                            self.rect.x = self.start_pos.0;
                            self.rect.y = self.start_pos.1;
                            return 1 + goal.dest;
                        }
                    }
                    Map::Spike(spike) => {
                        if let Some(..) = self.rect.intersection(spike.rect) {
                            if return_num == 0 {
                                return_num = -1;
                            }
                        }
                    }
                    _ => (),
                }
            }
            if return_num != 0 {
                break 'map;
            }
        }
        if return_num < 0 {
            self.rect.x = self.start_pos.0;
            self.rect.y = self.start_pos.1;
        }
        return_num
    }

    fn fall(&mut self) {
        self.vel.1 += 1;
    }

    fn ajust_pos(&mut self, tile: Rect) -> i8 {
        if self.previous_position.y + self.rect.h - 1 <= tile.y {
            self.rect.y = tile.y - self.rect.h + 1;
            self.vel.1 = 0;
            return 1;
        } else if self.previous_position.y >= tile.y + tile.h {
            self.rect.y = tile.y + tile.h + 1;
            self.vel.1 = 0;
            return 2;
        } else if self.previous_position.x >= tile.x + tile.w {
            self.vel.0 = 0;
            self.rect.x = tile.x + tile.w;
            return 0;
        } else if self.previous_position.x + self.rect.w <= tile.x {
            self.vel.0 = 0;
            self.rect.x = tile.x - self.rect.w;
            return 0;
        }
        self.rect.y = tile.y + tile.h + 1;
        self.vel.1 = 0;
        2
    }

    pub fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.animation_num += 1;
        if self.animation_num == 54 {
            self.animation_num = 0;
        }
        if self.animation_num < 49 {
            canvas
                .copy(&self.texture, Rect::new(0, 0, 16, 16), self.draw_rect)
                .unwrap();
        } else {
            canvas
                .copy(&self.texture, Rect::new(16, 0, 16, 16), self.draw_rect)
                .unwrap();
        }
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
