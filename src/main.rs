extern crate ncurses;

mod game;

use game::Game;
use ncurses::*;

fn main() {
    initscr();
    noecho();
    cbreak();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut y_max = 0;
    let mut x_max = 0;

    getmaxyx(stdscr(), &mut y_max, &mut x_max);

    let game_window = newwin(y_max / 2, x_max / 2, y_max / 4, x_max / 4);
    box_(game_window, 0, 0);

    let mut game = Game::init(game_window);
    game.start();

    endwin();
}
