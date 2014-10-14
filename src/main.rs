extern crate libc;

extern crate ncurses;

use std::io::Timer;
use std::time::Duration;

pub mod statgrab;
mod cpu;
mod disk;

fn main() {
    /* Setup ncurses. */
    ncurses::initscr();
    ncurses::raw();

    ncurses::noecho();
    // TODO get refresh rate from args
    ncurses::halfdelay(10);

    statgrab::init();

    // Invisible cursor.
    ncurses::curs_set(ncurses::CURSOR_INVISIBLE);

    let cpu_win = ncurses::newwin(3, ncurses::COLS, 0, 0);
    let disk_win = ncurses::newwin(3, ncurses::COLS, 1, 0);

    let mut ch = ncurses::getch();
    // loop until q is pressed
    while ch != 113 {
        match ch {
          _ => {}
        }
        cpu::draw_dashboard(cpu_win);
        disk::draw_dashboard(disk_win);
        ch = ncurses::getch();
    }

    ncurses::endwin();
}

fn destroy_win(win: ncurses::WINDOW) {
    let ch = ' ' as u32;
    ncurses::wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    ncurses::wrefresh(win);
    ncurses::delwin(win);
}
