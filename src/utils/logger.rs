use console::style;

pub fn error(msg: &str) {
    println!("{} - {}", style("error").red(), msg);
}

pub fn info(msg: &str) {
    println!("{}  - {}", style("info").cyan(), msg)
}

pub fn ready(msg: &str) {
    println!("{} - {}", style("ready").green(), msg);
}

pub fn pick(msg: &str) {
    println!("{}  - {}", style("pick").color256(69), msg);
}

pub fn event(msg: &str) {
    println!("{} - {}", style("event").magenta(), msg);
}

pub fn full_info(msg: &str) {
    println!("{}  - {}", style("info").cyan(), style(msg).yellow());
}

pub fn warning(msg: &str) {
    println!("{}  - {}", style("warn").yellow(), msg);
}