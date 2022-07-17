use std::io;
use std::{thread, thread::sleep, time::Duration};
use std;
use tui::{
    backend::CrosstermBackend,
    widgets::{ Widget, Block, Borders, canvas },
    layout::{ Layout, Constraint, Direction, Rect },
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct Game {
    window : Terminal<CrosstermBackend<io::Stdout>>,
    is_running : bool,
    x : u16,
    y : u16,
}
impl Game{
    pub fn update(&mut self){
        let input = self.get_user_input().expect("Failed to get input");
        if input == 'c' { self.is_running = false } 
        if input == 'w' { self.y = self.y - 1 }
        if input == 's' { self.y = self.y + 1 }
        if input == 'a' { self.x = self.x - 1 }
        if input == 'd' { self.x = self.x + 1 }
    }
    pub fn render(&mut self) -> Result<(), io::Error> {
        self.window.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);
            f.render_widget(block, size);
            let small_block = Block::default().borders(Borders::ALL);
            f.render_widget(small_block, 
                tui::layout::Rect::new(self.x,self.y,10,10));
        })?;
        return Ok(());
    }
    pub fn get_user_input(&mut self) -> Result<char, io::Error> {
        if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        return Ok('\n');
                    }
                    KeyCode::Char(c) => {
                        return Ok(c);
                    }
                    KeyCode::Backspace => {
                        return Ok(' ');
                    }
                    KeyCode::Esc => {
                        return Ok(' ');
                    }
                    _ => {
                        return Ok(' ');
                    }
            }
        }
        Ok(' ')
    }
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
  
    let mut game = Game {
       window : terminal,
       is_running : true,
       x : 0,
       y : 0,
    };
 
    while game.is_running {
        game.update();
        game.render()
            .expect("Bad shit happened doodling");
        // thread::sleep(Duration::new(2,0));
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        game.window.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    game.window.show_cursor()?;


    Ok(())
} 



