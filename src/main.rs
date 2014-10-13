extern crate libc;

extern crate ncurses;

use std::io::Timer;
use std::time::Duration;

pub mod cpu;
pub mod statgrab;


fn main() {
    /* Setup ncurses. */
    ncurses::initscr();
    ncurses::raw();

    /* Allow for extended keyboard (like F1). */
    ncurses::keypad(ncurses::stdscr, true);
    ncurses::noecho();

    /* Invisible cursor. */
    //ncurses::curs_set(ncurses::CURSOR_INVISIBLE);

    ncurses::refresh();


    let win: ncurses::WINDOW = create_win();
    cpu::draw_dashboard(win);

    let mut ch = ncurses::getch();
    // loop until q is pressed
    while ch != 113 {
        match ch {
          _ => {}
        }

        ch = ncurses::getch();
    }

    ncurses::endwin();
    /*
    let mut timer = Timer::new().unwrap();
    let periodic = timer.periodic(Duration::milliseconds(1000));
    loop {
        periodic.recv();
        let stats = statgrab::get_cpu_percents();
        ncurses::printw(format!("{} {} {} {} {} {}", stats.user, stats.kernel, stats.idle, stats.iowait, stats.swap, stats.nice).to_string().as_slice());
        ncurses::refresh();
    }*/


}

fn create_win() -> ncurses::WINDOW {
    let win = ncurses::newwin(ncurses::LINES / 3, ncurses::COLS / 3, ncurses::LINES / 2, ncurses::COLS / 2);
    ncurses::box_(win, 0, 0);
    ncurses::wrefresh(win);
    win
}

fn destroy_win(win: ncurses::WINDOW) {
    let ch = ' ' as u32;
    ncurses::wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    ncurses::wrefresh(win);
    ncurses::delwin(win);
}
