extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use piston::Events;

use std::collections::LinkedList;

const BACKGROUND_COLOR: [f32; 4] = [0.066, 0.066, 0.066, 0.0];
const SNAKE_BODY_COLOR: [f32; 4] = [0.752, 0.184, 0.258, 1.0];

#[derive(Clone, PartialEq)]
enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        self.gl.draw(arg.viewport(), |_, gl| {
            graphics::clear(BACKGROUND_COLOR, gl);
        });

        self.snake.redner(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
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

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

struct Pos {
    x: i32,
    y: i32,
}

impl Snake {
    fn redner(&self, gl: &mut GlGraphics, args: &RenderArgs) {
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

    fn update(&mut self) {
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

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [400, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::RIGHT,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_) = e.update_args() {
            game.update();
        }

        if let Some(key) = e.button_args() {
            if key.state == ButtonState::Press {
                // quit game if q key is pressed
                if key.button == Button::Keyboard(Key::Q) {
                    return;
                }

                game.pressed(&key.button);
            }
        }
    }
}
