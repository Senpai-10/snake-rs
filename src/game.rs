use ncurses::*;

pub struct Game {
    window: WINDOW,
}

impl Game {
    fn init(game_window: WINDOW) -> Game {
        Game {
            window: game_window,
        }
    }
}
