extern crate ncurses;

mod colors;
mod game;
mod help;
mod insert_new_head;
mod snake;

use colors::*;
use game::Game;
use ncurses::*;

fn main() {
    initscr();
    noecho();
    cbreak();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(SNAKE_HEAD, COLOR_BLACK, COLOR_WHITE);

    let mut y_max = 0;
    let mut x_max = 0;

    getmaxyx(stdscr(), &mut y_max, &mut x_max);

    let game_window = newwin(y_max / 2, x_max / 2, y_max / 4, x_max / 4);
    nodelay(game_window, true);

    let mut game = Game::init(game_window, x_max, y_max);
    game.start();
    endwin();
}
