use piston::input::*;
use opengl_graphics::GlGraphics;

pub mod player;

pub trait Entity {
    fn render(&mut self, args: &RenderArgs,gl : &mut GlGraphics,
            x_offset : f64,
            y_offset : f64);
    fn update(&mut self, e: &Event);
}
