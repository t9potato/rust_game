use sdl2::rect::Rect;

pub struct Ground {
    pub rect: Rect,
}

impl Ground {
    pub fn new(x: i32, y: i32) -> ground {
        ground {
            rect: Rect::new(x * 126, y * 126, 126, 126)
        }
    }
}
