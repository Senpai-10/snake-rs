use ncurses::{ll::getmaxy, *};

pub struct Game {
    window: WINDOW,
    min: WindowSize,
    max: WindowSize,
    score: u32,
    quit: bool,
}

struct WindowSize {
    x: i32,
    y: i32,
}

impl Game {
    pub fn init(game_window: WINDOW) -> Game {
        Game {
            window: game_window,
            min: WindowSize { x: 0, y: 0 },
            max: WindowSize { x: 0, y: 0 },
            score: 0,
            quit: false,
        }
    }

    pub fn start(&mut self) {
        self.set_min_and_max();

        // main loop
        while !self.quit {
            self.display_score();
            wmove(self.window, self.min.y, self.min.x);
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

    fn set_min_and_max(&mut self) {
        self.min.x = 1;
        self.min.y = 1;

        getmaxyx(self.window, &mut self.max.y, &mut self.max.x);

        // subtract 1 from max x, y
        // Because the max value is on the border line
        self.max.x = self.max.x - 1;
        self.max.y = self.max.y - 1;
    }
}
