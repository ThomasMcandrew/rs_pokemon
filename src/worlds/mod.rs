use piston::input::*;
use opengl_graphics::GlGraphics;

pub struct World {
    width : i32,
    height : i32,

}
impl World {
    pub fn new() -> World {
        World {
            width : 500,
            height : 500,
        }
    }
    pub fn render(&mut self, args: &RenderArgs,gl : &mut GlGraphics) {
        use graphics::*;
       
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = rectangle::square(0.0, 0.0, 50.0);
        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform
                .trans(100.0, 100.0)
                .trans(-25.0, -25.0);
            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }
}
