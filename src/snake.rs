use crate::colors::*;
use crate::constants::{SNAKE_BODY_CHAR, SNAKE_HEAD_CHAR};
use crate::game::WindowSize;
use ncurses::*;

pub struct Snake {
    pub body: Vec<(i32, i32)>,
    window: WINDOW,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Snake {
    pub fn new(window: WINDOW) -> Snake {
        Snake {
            body: vec![(4, 10), (4, 9), (4, 8)],
            window: window,
        }
    }

    pub fn render(&self) {
        for (index, part) in self.body.iter().enumerate() {
            wmove(self.window, part.0, part.1);
            if index == 0 {
                wattron(self.window, COLOR_PAIR(SNAKE_HEAD));
                waddch(self.window, SNAKE_HEAD_CHAR as u32);
                wattr_off(self.window, COLOR_PAIR(SNAKE_HEAD));
            } else {
                waddch(self.window, SNAKE_BODY_CHAR as u32);
            }
        }
    }

    pub fn move_head(
        &mut self,
        direction: Direction,
        current_direction: Direction,
        min: &WindowSize,
        max: &WindowSize,
    ) {
        let mut x = self.body[0].1;
        let mut y = self.body[0].0;

        match direction {
            Direction::UP => {
                if y == min.y {
                    return;
                }

                y -= 1
            }
            Direction::DOWN => {
                if y == max.y {
                    return;
                }

                y += 1
            }
            Direction::RIGHT => {
                if x == max.x {
                    return;
                }

                x += 1
            }
            Direction::LEFT => {
                if x == min.x {
                    return;
                }

                x -= 1
            }
        }

        self.body.insert(0, (y, x));
    }

    /// get snake head coordinates (y, x)
    pub fn get_head(&self) -> (i32, i32) {
        return self.body[0];
    }
}
