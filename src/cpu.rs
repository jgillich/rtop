extern crate ncurses;

pub fn draw_dashboard(win: ncurses::WINDOW) {
    let stats = statgrab::get_cpu_percents();
    win.wprintw(format!("{} {} {} {} {} {}", stats.user, stats.kernel, stats.idle, stats.iowait, stats.swap, stats.nice).to_string().as_slice());
}
