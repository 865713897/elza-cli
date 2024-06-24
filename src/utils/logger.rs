use console::style;

pub fn error(msg: &str) {
    println!("{}  - {}", style("error").red(), msg);
}

pub fn info(msg: &str) {
    println!("{}   - {}", style("info").cyan(), msg)
}

pub fn ready(msg: &str) {
    println!("{}  - {}", style("ready").green(), msg);
}

pub fn select_msg(msg: &str) {
    println!("{} - {}", style("select").color256(33), msg);
}

pub fn event(msg: &str) {
    println!("{}  - {}", style("event").color256(177), msg);
}

pub fn link_info(msg: &str) {
    println!("{}   - {}", style("link").color256(33), style(msg).color256(33).underlined());
}

pub fn warn(msg: &str) {
    println!("{}  - {}", style("warn").color256(226), msg);
}
