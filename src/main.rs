mod app;

use app::App;
use log::debug;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Debug).expect("init log");
    debug!("Init logging");
    yew::start_app::<App>();
}
