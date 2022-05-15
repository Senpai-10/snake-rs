extern crate ncurses;

mod game;

use game::Game;
use ncurses::*;

struct Window {
    y_max: i32,
    x_max: i32,
}

fn main() {
    initscr();
    noecho();
    cbreak();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut window = Window { x_max: 0, y_max: 0 };
    getmaxyx(stdscr(), &mut window.y_max, &mut window.x_max);

    let game_window = newwin(
        window.y_max / 2,
        window.x_max / 2,
        window.y_max / 4,
        window.x_max / 4,
    );
    box_(game_window, 0, 0);

    let mut game = Game::init(game_window);
    game.start();

    endwin();
}
