use piston::input::*;
use opengl_graphics::GlGraphics;

pub mod player;

pub struct Doodle {
    pub x : f64,
    pub y : f64,
    pub width : f64,
    pub height : f64,
    pub sprite : i32,
}
pub trait Entity {
    fn render(&mut self, args: &RenderArgs,gl : &mut GlGraphics);
    fn update(&mut self, e: &Event);
}
