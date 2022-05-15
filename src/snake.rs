use ncurses::*;

pub struct Snake {
    pub body: Vec<(i32, i32)>,
    window: WINDOW,
}

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
        for part in self.body.iter() {
            wmove(self.window, part.0, part.1);
            waddch(self.window, '#' as u32);
        }
    }

    pub fn move_head(&mut self, direction: Direction) {
        let mut x = self.body[0].1;
        let mut y = self.body[0].0;

        match direction {
            Direction::UP => y -= 1,
            Direction::DOWN => y += 1,
            Direction::RIGHT => x += 1,
            Direction::LEFT => x -= 1,
        }

        let mut new_body: Vec<(i32, i32)> = Vec::new();

        new_body.push((y, x));

        let mut old_body = self.body.clone();

        new_body.append(&mut old_body);

        self.body = new_body;

        // self.body.append(&mut [(self.body[0].0, self.body[0].1)]);
    }
    // pub fn get_snake_head(&self) -> (i32, i32) {
    //     return (self.body[0].0, self.body[0].1);
    // }
}
