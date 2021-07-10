#[allow(unused)]
pub mod light {
    pub fn rect(width: i32, height: i32, color: sdl2::pixels::Color, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> sdl2::render::Texture {
        let mut surface = sdl2::surface::Surface::new(width as u32, height as u32, sdl2::pixels::PixelFormatEnum::RGB888).unwrap();
        surface.fill_rect(sdl2::rect::Rect::new(0,0,width as u32,height as u32), color).unwrap();
        let mut texture = surface.as_texture(texture_creator).unwrap();
        texture.set_blend_mode(sdl2::render::BlendMode::Add);
        texture
    }

    pub fn circle(radius: i32, color: sdl2::pixels::Color, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> sdl2::render::Texture {
        use sdl2::gfx::primitives::DrawRenderer;
        let mut surf_canvas = sdl2::surface::Surface::new(radius as u32 * 2, radius as u32 * 2, sdl2::pixels::PixelFormatEnum::RGB888).unwrap().into_canvas().unwrap();
        surf_canvas.filled_circle(radius as i16, radius as i16, radius as i16, color).unwrap();
        surf_canvas.present();
        let surface = surf_canvas.into_surface();
        let mut texture = surface.as_texture(texture_creator).unwrap();
        texture.set_blend_mode(sdl2::render::BlendMode::Add);
        texture
    }
}

pub mod particles {
    pub struct Full {
        x: f32,
        y: f32,
        vel_x: f32,
        vel_y: f32,
        radius: f32,
        color: sdl2::pixels::Color,
    }

    impl Full {
        pub fn new(x: f32, y: f32,  vel_x: f32, vel_y: f32, radius: f32, color: sdl2::pixels::Color) -> Full {
            Full {
                x,
                y,
                vel_x,
                vel_y,
                radius,
                color
            }
        }

        pub fn update(&mut self) -> bool{
            self.x += self.vel_x;
            self.y -= self.vel_y;
            self.radius -= 0.05;
            if self.radius <= 1.0 {
                return true;
            }
            false
        }

        pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
            use sdl2::gfx::primitives::DrawRenderer;
            canvas.filled_circle(((self.x + self.radius * 4.0) as i32) as i16, ((self.y + self.radius * 4.0) as i32) as i16, (self.radius * 4.0) as i16, self.color).unwrap();
        }
    }
}
