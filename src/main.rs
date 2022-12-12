mod ui;
mod player;
mod queue;
mod app;
mod structs;

use gtk::prelude::*;
use crate::ui::BeatWindow;
use crate::app::BeatApp;

const APP_ID: &str = "ru.slie.beat";


fn main() {
    gtk::init().unwrap();
    let app = BeatApp::new(APP_ID);
    ui::cli::make_cli(&app);

    app.run();
}
