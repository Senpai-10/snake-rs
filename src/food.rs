use ncurses::*;
use rand::Rng;

use crate::game::WindowSize;

pub struct Food {
    window: WINDOW,
    min: WindowSize,
    max: WindowSize,
    pub coordinates: (i32, i32), // 0: y, 1: x
}

impl Food {
    pub fn init(window: WINDOW, min: WindowSize, max: WindowSize) -> Food {
        Food {
            window: window,
            min: min,
            max: max,
            coordinates: (0, 0),
        }
    }

    pub fn generate(&mut self) {
        let random_x: i32 = rand::thread_rng().gen_range(self.min.x..self.max.x);
        let random_y: i32 = rand::thread_rng().gen_range(self.min.y..self.max.y);

        wmove(self.window, random_y, random_x);
        waddch(self.window, 'f' as u32);

        self.coordinates = (random_y, random_x)
    }
}
