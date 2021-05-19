use crate::ground::*;
use sdl2::rect::Rect;

pub struct Vec2(pub i32, pub i32);

pub struct Player {
    pub rect: Rect,
    previous_position: Rect,
    draw_rect: Rect,
    vel: Vec2,
    min_vel: Vec2,
    max_vel: Vec2,
    pub input: i8,
    pub jump: bool,
    grounded: bool,
}

impl Player {
    pub fn new(rect: Rect) -> Player {
        Player {
            draw_rect: Rect::new(rect.x, rect.y + 1, rect.width(), rect.height()),
            rect: Rect::new(rect.x / 4, rect.y / 4, rect.width() / 4, rect.height() / 4),
            previous_position: Rect::new(rect.x / 4, rect.y / 4, rect.width() / 4, rect.height() / 4),
            vel: Vec2(0, 0),
            min_vel: Vec2(-2, -7),
            max_vel: Vec2(2, 7),
            input: 0,
            jump: false,
            grounded: false,
        }
    }

    pub fn update(&mut self, map: &Vec<Vec<Map>>) {
        match self.input {
            1 => {
                self.vel.0 += 1
            },
            0 => {
                if self.vel.0 < 0 {
                    self.vel.0 += 1;
                } else if self.vel.0 > 0 {
                    self.vel.0 -= 1;
                }
            },
            -1=> {
                self.vel.0 -=1
            },
            _ => (),
        }


        self.vel = clamp(&self.vel, &self.min_vel, &self.max_vel);
        self.mov_pos();
        self.grounded = self.grounded(&map);
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
    }

    fn mov_pos(&mut self) {
        self.rect.x += self.vel.0;
        self.rect.y += self.vel.1;
    }

    fn jump(&mut self) {
         self.vel.1 = -7;
    }

    fn grounded(&mut self, tiles: &Vec<Vec<Map>>) -> bool {
        for rows in tiles {
            for tile in rows {
                match tile {
                        Map::Ground(floor) => {
                            if let Some(..) = self.rect.intersection(floor.rect) {
                                if self.ajust_pos(floor.rect) {
                                    return true;
                                }
                            }},
                        Map::Air => (),
                        
                    }
                }
            }
        false
    }

    fn fall(&mut self) {
        self.vel.1 += 1;
    }

    fn ajust_pos(&mut self, tile: Rect) -> bool {
        if self.previous_position.y + self.rect.h -1 <= tile.y {
            self.rect.y = tile.y - self.rect.h + 1;
            return true;
        } else if self.previous_position.y >= tile.y + tile.h {
            self.rect.y = tile.y + tile.h + 1;
            self.vel.1 = 0;
            return false;
        } else if self.previous_position.x >= tile.x + tile.w {
            self.vel.0 = 0;
            self.rect.x = tile.x + tile.w;
            return false;
        }
        self.vel.0 = 0;
        self.rect.x = tile.x - self.rect.w;
        return false;
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(32, 32, 32));
        canvas.fill_rect(self.draw_rect).unwrap();
    }
}

fn clamp(num: &Vec2, min: &Vec2, max: &Vec2) -> Vec2{
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
