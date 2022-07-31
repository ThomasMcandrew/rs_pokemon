use piston::input::*;
use opengl_graphics::GlGraphics;

pub struct Tile {
    color : [f32; 4],
}
impl Tile {
    pub fn new() -> Tile {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        Tile { 
            color : [rng.gen(),rng.gen(),rng.gen(),rng.gen()],
        }
    }
    pub fn get_tile_width() -> f64 {
        32.0
    }
    pub fn render(&mut self, 
            args: &RenderArgs,
            gl : &mut GlGraphics,
            x : f64,
            y : f64) {
        use graphics::*;
        gl.draw(args.viewport(), |c, gl| {
            rectangle(self.color, 
                rectangle::square(0.0,0.0,Tile::get_tile_width()),
                c.transform
                .trans(x,y),gl);
        });

    } 
}
