use crate::colors::*;
use opengl_graphics::GlGraphics;
use piston::input::*;
use std::collections::LinkedList;
#[derive(Clone, PartialEq)]
pub enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

pub struct Snake {
    pub body: LinkedList<(i32, i32)>,
    pub dir: Direction,
}

impl Snake {
    pub fn redner(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        // let square =
        //     graphics::rectangle::square((self.pos.x * 20) as f64, (self.pos.y * 20) as f64, 20_f64);

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter().for_each(|square| {
                graphics::rectangle(SNAKE_BODY_COLOR, square, transform, gl);
            })
        });
    }

    pub fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::LEFT => new_head.0 -= 1,
            Direction::RIGHT => new_head.0 += 1,
            Direction::UP => new_head.1 -= 1,
            Direction::DOWN => new_head.1 += 1,
        }

        self.body.push_front(new_head);
        self.body.pop_back();
    }
}
