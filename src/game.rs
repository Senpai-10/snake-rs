use crate::colors::*;
use crate::snake::{Direction, Snake};
use opengl_graphics::GlGraphics;
use piston::input::*;

pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
}

impl Game {
    pub fn render(&mut self, arg: &RenderArgs) {
        self.gl.draw(arg.viewport(), |_, gl| {
            graphics::clear(BACKGROUND_COLOR, gl);
        });

        self.snake.redner(&mut self.gl, arg);
    }

    pub fn update(&mut self) {
        self.snake.update();
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();
        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            _ => last_direction,
        };
    }
}
