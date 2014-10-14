extern crate ncurses;

use statgrab;

pub fn draw_dashboard(win: ncurses::WINDOW) {
    ncurses::wclear(win);
    let stats = statgrab::get_cpu_percents();
    ncurses::wprintw(win, format!("CPU {}%%", 100.0 - stats.idle).to_string().as_slice());
    ncurses::wrefresh(win);
}
