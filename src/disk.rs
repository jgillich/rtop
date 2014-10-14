extern crate ncurses;

use statgrab;

pub fn draw_dashboard(win: ncurses::WINDOW) {
    ncurses::wclear(win);
    let stats = statgrab::get_disk_io_stats_diff();
    let i = 1;
    ncurses::wprintw(win, format!("DISK {} {} {}", stats[i].disk_name, stats[i].read_bytes / stats[i].systime, stats[i].write_bytes / stats[i].systime).to_string().as_slice());
    ncurses::wrefresh(win);
}
