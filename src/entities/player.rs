use opengl_graphics::GlGraphics;
use crate::entities::Entity;

use piston::input::*;

pub struct Player {
    pub x : f64,
    pub y : f64,
    pub width : f64,
    pub height : f64,
    pub speed : f64, 
    pub inputs : Vec<bool>,
}
enum Inputs {
    Left,
    Right,
    Up,
    Down,
}
impl Inputs {
    pub fn get_index(&mut self) -> usize {
        match self {
            Inputs::Down    => 0,
            Inputs::Up      => 1,
            Inputs::Left    => 2,
            Inputs::Right   => 3,
        }
    }
}
impl Player {
    pub fn new() -> Player {
        Player {
            x : 0.0,
            y : 0.0,
            width : 32.0,
            height : 32.0,
            speed : 1.0,
            inputs : vec![false,false,false,false],
        }
    }
    fn get_player_dir(&mut self) -> (f64,f64){
        let xdir = if self.inputs[Inputs::Left.get_index()] && 
                        self.inputs[Inputs::Right.get_index()]
                        { 0.0 }
                    else if self.inputs[Inputs::Left.get_index()]
                        { -1.0 }
                    else if self.inputs[Inputs::Right.get_index()]
                        { 1.0 }
                    else { 0.0 };
        let ydir = if self.inputs[Inputs::Up.get_index()] && 
                        self.inputs[Inputs::Down.get_index()]
                        { 0.0 }
                    else if self.inputs[Inputs::Up.get_index()]
                        { -1.0 }
                    else if self.inputs[Inputs::Down.get_index()]
                        { 1.0 }
                    else { 0.0 };
        (xdir,ydir)
    }
    fn handle_user_input(&mut self, e: &Event) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => self.inputs[Inputs::Up.get_index()]       = true,
                Key::S => self.inputs[Inputs::Down.get_index()]     = true,
                Key::A => self.inputs[Inputs::Left.get_index()]     = true,
                Key::D => self.inputs[Inputs::Right.get_index()]    = true,
                _ => (), 
            }
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::W => self.inputs[Inputs::Up.get_index()]       = false,
                Key::S => self.inputs[Inputs::Down.get_index()]     = false,
                Key::A => self.inputs[Inputs::Left.get_index()]     = false,
                Key::D => self.inputs[Inputs::Right.get_index()]    = false,
                _ => (), 
            }
        }
    }
}

impl Entity for Player {
    fn render(&mut self, args: &RenderArgs,
            gl : &mut GlGraphics,
            x_offset : f64,
            y_offset : f64) {
        use graphics::*;
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        gl.draw(args.viewport(), |c, gl| {
            rectangle(RED, 
                rectangle::square(0.0,0.0,self.width),
                c.transform
                .trans(self.x - x_offset,
                    self.y - y_offset),
                gl);
        });
    }
    fn update(&mut self, e: &Event) {
        self.handle_user_input(e);
        let (xdir,ydir) = self.get_player_dir();
        self.x = self.x + (self.speed * xdir);
        self.y = self.y + (self.speed * ydir);
   }
}
