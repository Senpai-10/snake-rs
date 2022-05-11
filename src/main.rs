extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod colors;
mod game;
mod snake;

use game::Game;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use piston::Events;
use snake::{Direction, Snake};

use std::collections::LinkedList;

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
