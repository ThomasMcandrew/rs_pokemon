extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod entities;
mod worlds;
use crate::entities::player::Player;
use crate::entities::Entity;
use crate::worlds::World;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    player : Player,
    world : World,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |_, gl| {
            clear(GREEN, gl);
       });
        self.world.render(args, &mut self.gl);
        self.player.render(args, &mut self.gl)
    }
    fn update(&mut self, e: &Event) {
         if let Some(args) = e.update_args() {
            self.rotation += 2.0 * args.dt;
         }
         self.player.update(e);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        player: Player::new(),
        world : World::new(),
   };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }    
        game.update(&e);
    }
}
