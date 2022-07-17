extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    playerx : f64,
    playery : f64,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let square2 = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);
            let transform2 = c
                .transform
                .trans(self.playerx,self.playery);
            rectangle(RED, square2, transform2, gl);
            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }
    fn update(&mut self, e: &Event) {
        // Rotate 2 radians per second.
        if let Some(args) = e.update_args() {
            self.rotation += 2.0 * args.dt;
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => self.playery = self.playery - 1.0,
                Key::S => self.playery = self.playery + 1.0,
                Key::A => self.playerx = self.playerx - 1.0,
                Key::D => self.playerx = self.playerx + 1.0,
                _ => (), 
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => self.playery = self.playery - 1.0,
                Key::S => self.playery = self.playery + 1.0,
                Key::A => self.playerx = self.playerx - 1.0,
                Key::D => self.playerx = self.playerx + 1.0,
                _ => (), 
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        playerx : 0.0,
        playery : 0.0, 
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }    
        game.update(&e);
    }
}
