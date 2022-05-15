use ncurses::*;

pub fn display_help_box(x_max: i32, y_max: i32) {
    let help_window = newwin(y_max / 4, x_max / 4 / 2, y_max / 2, x_max - 50);
    let mut help_x_max = 0;
    let mut help_y_max = 0;
    getmaxyx(help_window, &mut help_y_max, &mut help_x_max);

    box_(help_window, 0, 0);

    wmove(help_window, 0, (help_x_max - 4) / 2);
    waddstr(help_window, "Help");

    print_in_center(help_window, help_x_max, help_y_max / 4, "Up:   w, ^");
    print_in_center(help_window, help_x_max, (help_y_max / 4) + 2, "Down: s, >");
    print_in_center(help_window, help_x_max, (help_y_max / 4) + 4, "Left: a, <");
    print_in_center(help_window, help_x_max, (help_y_max / 4) + 6, "Right: d, v");
    wrefresh(help_window);
}

fn print_in_center(window: WINDOW, x_max: i32, y: i32, message: &str) {
    wmove(window, y, (x_max - message.len() as i32) / 2);
    waddstr(window, message);
}
