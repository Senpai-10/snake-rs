use crate::help::display_help_box;
use ncurses::*;

pub struct Game {
    window: WINDOW,
    min: WindowSize,
    max: WindowSize,
    screen_x_max: i32,
    screen_y_max: i32,
    score: u32,
    quit: bool,
}

struct WindowSize {
    x: i32,
    y: i32,
}

impl Game {
    pub fn init(game_window: WINDOW, screen_x_max: i32, screen_y_max: i32) -> Game {
        Game {
            window: game_window,
            min: WindowSize { x: 0, y: 0 },
            max: WindowSize { x: 0, y: 0 },
            screen_x_max: screen_x_max,
            screen_y_max: screen_y_max,
            score: 0,
            quit: false,
        }
    }

    pub fn start(&mut self) {
        self.set_min_and_max();

        // main loop
        while !self.quit {
            display_help_box(self.screen_x_max, self.screen_y_max);
            box_(self.window, 0, 0); // draw the game window border

            self.display_score();
            self.handle_input();
            refresh();
        }
    }

    fn display_score(&self) {
        let score = &format!("score: {} ", self.score);

        wmove(self.window, 0, (self.max.x - score.len() as i32) / 2);
        waddstr(self.window, score);
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
