mod database;
mod listener;
mod render;
mod tui;


fn main() {
    listener::start_listener();
    tui::start_tui().unwrap();
}
