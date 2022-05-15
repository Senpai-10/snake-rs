use ncurses::*;

pub struct Game {
    window: WINDOW,
    score: u32,
    quit: bool,
}

impl Game {
    pub fn init(game_window: WINDOW) -> Game {
        Game {
            window: game_window,
            score: 0,
            quit: false,
        }
    }

    pub fn start(&mut self) {
        // main loop
        while !self.quit {
            self.display_score();
            wmove(self.window, 1, 1);
            waddstr(self.window, "hi");
            self.handle_input();
        }
    }

    fn display_score(&self) {
        wmove(self.window, 0, 0);
        waddstr(self.window, &format!("score: {} ", self.score));
    }

    fn handle_input(&mut self) {
        match wgetch(self.window) as u8 as char {
            'q' => self.quit = true,
            _ => {}
        }
    }
}
