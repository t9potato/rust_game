use sdl2::rect::Rect;

pub struct Vec2(pub i32, pub i32);

pub struct Player {
    pub rect: Rect,
    pub vel: Vec2,
    pub min_vel: Vec2,
    pub max_vel: Vec2,
    pub input: i8,
}

impl Player {
    pub fn new(rect: Rect) -> Player {
        Player {
            rect,
            vel: Vec2(0, 0),
            min_vel: Vec2(-5, -5),
            max_vel: Vec2(5, 5),
            input: 0,
        }
    }
    pub fn update(&mut self) {
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
        self.walk();
    }
    fn walk(&mut self) {
        self.vel = clamp(&self.vel, &self.min_vel, &self.max_vel);
        self.rect.x += self.vel.0;
        self.rect.y += self.vel.1;
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
