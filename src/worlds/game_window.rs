pub struct GameWindow {
   pub x_offset : f64,
   pub y_offset : f64,
}

impl GameWindow {
    pub fn new() -> GameWindow {
        GameWindow {  
            x_offset : 0.0,
            y_offset : 0.0,
        }
    }
    pub fn update_offset(&mut self, x : f64, 
        y : f64, 
        window_width : f64, 
        window_height : f64,
        game_width : f64,
        game_height : f64){
        self.x_offset = x - window_width / 2.0;
        self.y_offset = y - window_height / 2.0;
        if self.x_offset < 0.0 { self.x_offset = 0.0 } 
        if self.y_offset < 0.0 { self.y_offset = 0.0 } 
        if self.x_offset > game_width - window_width { 
            self.x_offset = game_width - window_width }
        if self.y_offset > game_height - window_height { 
            self.y_offset = game_height - window_height}

    }
}
