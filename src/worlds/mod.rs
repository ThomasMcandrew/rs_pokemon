pub mod game_window;
pub mod tile;

use crate::worlds::game_window::GameWindow;
use crate::worlds::tile::Tile;
use crate::Player;
use crate::entities::Entity;

use piston::input::*;
use opengl_graphics::GlGraphics;

pub struct World {
    width : i32,
    height : i32,
    tile_size : i32,
    data : Vec<Tile>,
    game_window : GameWindow,
    player : Player,
}
impl World {
    pub fn new() -> World {
        let mut d : Vec<Tile> = Vec::new();
        for _ in 0..(50 * 50) {
            d.push(Tile::new());
        }
        World {
            width : 50,
            height : 50,
            tile_size : 50,
            data : d,
            game_window : GameWindow::new(),
            player : Player::new(),
        }
    }
    pub fn render(&mut self, 
            args: &RenderArgs,
            gl : &mut GlGraphics) {
        let window_width = 50.0;
        let window_height = 50.0;
        self.game_window.update_offset(self.player.x,
            self.player.y,
            window_width,
            window_height,
            self.width as f64 * Tile::get_tile_width(),
            self.height as f64 * Tile::get_tile_width());
        let (mut x_offset,mut y_offset) = (self.game_window.x_offset,
                                    self.game_window.y_offset);
        let tile_width = Tile::get_tile_width();
        x_offset = x_offset ;
        y_offset = y_offset ;
        let mut x_end = x_offset + window_width / tile_width + 1.0;
        let mut y_end = y_offset + window_height / tile_width + 1.0;
        if x_end > window_width { x_end = window_width; } 
        if y_end >= window_height { y_end = window_height; } 
        //make sure to cap x end and y end
        for y in 0..y_end as i32 {
            for x in 0..x_end as i32 {
                println!("{:?}",((((y * self.width) + x) as f64),
                    (((y * self.width) + x) as f64) <
                    (window_height * window_width) as f64,
                    (window_height * window_width)
                        ));
                if (((y * self.width) + x) as f64) <
                    (window_height * window_width)  {
                    self.data[(((y * self.width) + x) as usize)]
                        .render(args,gl,
                            (x as f64 * tile_width) - x_offset,
                            (y as f64 * tile_width) - y_offset);
   
                }
            }
        }
        self.player.render(args, gl,x_offset,y_offset);
    }
    pub fn update(&mut self, e: &Event){
        self.player.update(e);
    }
}
