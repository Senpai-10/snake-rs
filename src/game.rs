use crate::constants::SNAKE_HEAD_CHAR;
use crate::food::Food;
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
    direction: Direction,
    score: u32,
    quit: bool,
}

#[derive(Clone, Copy)]
pub struct WindowSize {
    pub x: i32,
    pub y: i32,
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
            direction: Direction::RIGHT,
            score: 0,
            quit: false,
        }
    }

    pub fn start(&mut self) {
        self.set_min_and_max();
        let mut food = Food::init(self.window, self.min, self.max);
        food.generate();

        // main loop
        while !self.quit {
            display_help_box(self.screen_x_max, self.screen_y_max);
            box_(self.window, 0, 0);
            self.display_score();

            if self.snake.get_head() == food.coordinates {
                self.score += 1;
                wmove(self.window, food.coordinates.0, food.coordinates.1);
                waddch(self.window, SNAKE_HEAD_CHAR as u32);

                food.generate();
            } else {
                let last_part = self.snake.body.pop().unwrap();
                wmove(self.window, last_part.0, last_part.1);
                waddch(self.window, ' ' as u32);
            }

            self.snake.render();

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
            'w' => {
                self.snake
                    .move_head(Direction::UP, self.direction, &self.min, &self.max);
                self.direction = Direction::UP;
            }
            's' => {
                self.snake
                    .move_head(Direction::DOWN, self.direction, &self.min, &self.max);
                self.direction = Direction::DOWN;
            }
            'd' => {
                self.snake
                    .move_head(Direction::RIGHT, self.direction, &self.min, &self.max);
                self.direction = Direction::RIGHT;
            }
            'a' => {
                self.snake
                    .move_head(Direction::LEFT, self.direction, &self.min, &self.max);
                self.direction = Direction::LEFT;
            }
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
        self.max.x = self.max.x - 2;
        self.max.y = self.max.y - 2;
    }
}
