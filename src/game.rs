use crate::help::display_help_box;
use crate::snake::{Direction, Snake};
use ncurses::*;

pub struct Game {
    window: WINDOW,
    min: WindowSize,
    max: WindowSize,
    screen_x_max: i32,
    screen_y_max: i32,
    snake: Snake,
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
            snake: Snake::new(game_window),
            score: 0,
            quit: false,
        }
    }

    pub fn start(&mut self) {
        self.set_min_and_max();

        // main loop
        while !self.quit {
            timeout((150 - (self.snake.body.len()) / 5 + self.snake.body.len() / 10 & 120) as i32);
            display_help_box(self.screen_x_max, self.screen_y_max);
            box_(self.window, 0, 0); // draw the game window border

            self.handle_input();
            self.snake.render();

            self.display_score();
            refresh();
        }
    }

    fn display_score(&self) {
        let score = &format!("score: {} ", self.score);

        wmove(self.window, 0, (self.max.x - score.len() as i32) / 2);
        waddstr(self.window, score);
    }

    fn handle_input(&mut self) {
        let x = self.snake.body[0].1;
        let y = self.snake.body[0].0;

        match wgetch(self.window) as u8 as char {
            'w' => {
                if y != self.min.y {
                    self.snake.move_head(Direction::UP)
                }
            }
            's' => {
                if y != self.max.y {
                    self.snake.move_head(Direction::DOWN)
                }
            }
            'd' => {
                if x != self.max.x {
                    self.snake.move_head(Direction::RIGHT)
                }
            }
            'a' => {
                if x != self.min.x {
                    self.snake.move_head(Direction::LEFT)
                }
            }
            'q' => self.quit = true,
            _ => {
                // self.snake.move_head(Direction::RIGHT);
            }
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
