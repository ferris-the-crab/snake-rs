// Code is a mess... I know :(

use color_eyre::{
    Result, eyre::WrapErr,
};

use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
};

use ratatui::{
    DefaultTerminal,
    Frame,
    widgets::{Block, Borders, Widget, Paragraph},
    layout::{Rect, Alignment},
    buffer::Buffer,
    style::{Style, Color},
};

use std::{time::Duration, thread::sleep};

const DELAY: u64 = 100;     // NOTE: Delay between every frame
const DEBUG: bool = true;   // NOTE: Prints some debug messages

#[derive(Debug)]
struct Player {
    sprite: String,
    x: u16,
    y: u16,
    direction: Direction,
}

#[derive(Debug)]
struct Area {
    x: u16,
    y: u16,
}

#[derive(Debug)]
struct Game {
    exit: bool,
    frame: u64,
    player: Player,
    area: Area,
}


// TODO: Add directions and basic snake movement
// NOTE: DONE!
#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


impl Default for Game {
    fn default() -> Self {
        Self {
            exit: false,
            frame: 0,
            player: Player {
                sprite: String::from("O"),
                x: 20,
                y: 20,
                direction: Direction::Down,
            },
            area: Area {
                x: 100,
                y: 50,
            },
        }
    }
}


impl Game {
    fn new(s: &str, x: u16, y: u16, maxx: u16, maxy: u16, direction: Direction) -> Self {
        Self {
            exit: false,
            frame: 0,
            player: Player {
                sprite: String::from(s),
                x, y, direction,
            },
            area: Area {
                x: maxx,
                y: maxy,
            }
        }
    }

    // NOTE: Game update loop
    fn update(&mut self) {
        self.frame += 1;
        match self.player.direction {
            Direction::Up => self.player.y -= 1,
            Direction::Down => self.player.y += 1,
            Direction::Left => self.player.x -= 1,
            Direction::Right => self.player.x += 1,
        }

        match self.player.y {
            0 => self.exit(),
            limit if limit == self.area.y => self.exit(),
            _ => {},
        }

        match self.player.x {
            0 => self.exit(),
            limit if limit == self.area.x => self.exit(),
            _ => {},
        }
    }

    // NOTE: Call this to exit using self.exit() in an implementation
    fn exit(&mut self) {
        self.exit = true;
    }
    
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.update();
        } Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        // NOTE: Check if an active event is a keypress
        if event::poll(Duration::from_millis(DELAY))? {
            sleep(Duration::from_millis(DELAY));
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    return self.handle_key_event(key_event)
                    .wrap_err_with(|| format!("Handling key event failed:\n{key_event:#?}"))
                } _ => return Ok(())
            }
        } Ok(())
    }

    // TODO: Implement Snake-like movement system 
    // use the 'Direction' enum and self.direction
    // NOTE: DONE!
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            // NOTE: Press 'q' to exit
            KeyCode::Char('q') => self.exit(),

            // NOTE: Simple movement, but it's not Snake-like... yet...
            KeyCode::Char('w') => { if self.player.direction != Direction::Down { self.player.direction = Direction::Up }},
            KeyCode::Char('s') => { if self.player.direction != Direction::Up { self.player.direction = Direction::Down }},
            KeyCode::Char('a') => { if self.player.direction != Direction::Right { self.player.direction = Direction::Left }},
            KeyCode::Char('d') => { if self.player.direction != Direction::Left { self.player.direction = Direction::Right }},
            _ => {}
        } Ok(())
    }
}


// NOTE: Utility crate
// Can be useful for a little bit of everything
mod utils {
    use ratatui::{
        layout::Rect,
        widgets::Paragraph,
        layout::Alignment,
    };


    pub fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
        let x = r.x.saturating_add((r.width.saturating_sub(width)) / 2);
        let y = r.y.saturating_add((r.height.saturating_sub(height)) / 2);
        Rect::new(x, y, width, height)
    }

    pub fn text(s: &str) -> Paragraph<'_> {
        Paragraph::new(s).alignment(Alignment::Left)
    }
}


// NOTE: The thing that actually shows on screen
// Frontend or whatever you wanna call it
impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let play_area = utils::centered_rect(self.area.x, self.area.y, area);
        let sprite_x = play_area.x + self.player.x;
        let sprite_y = play_area.y + self.player.y;
        let border = Block::default()
                .title("SNAKE")
                .borders(Borders::ALL)
                .border_style(Style::default()
                .fg(Color::Green));

        // NOTE: Make sure that the program doesn't panic, and the player stays within bounds
        if self.area.x < area.x + area.width && self.area.y < area.y + area.height {
            buf[(sprite_x, sprite_y)].set_symbol(&self.player.sprite);
            border.render(play_area, buf);
            if DEBUG {
                utils::text(&format!("Frame: {}", self.frame)).render(area, buf);
            }
        } else {
            // NOTE: If the terminal window is too small, stop rendering and display a warning
            // TODO: Pause the game in this case (I need to implement a self.pause() method)
            utils::text("Window is too small").render(area, buf);
        }
    }
}


// NOTE: Just the main() function
// Everything starts here
fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = Game::default().run(&mut terminal);
    ratatui::restore();
    match app_result {
        Ok(()) => {
            if DEBUG {
                println!("Game stopped successfully");
            }
        },
        _ => {},
    } app_result
}
