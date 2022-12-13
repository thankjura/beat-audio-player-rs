mod ui;
mod player;
mod app;
mod structs;

use gettextrs::*;
use gtk::prelude::*;
use crate::ui::BeatWindow;
use crate::app::BeatApp;

const APP_ID: &str = "ru.slie.beat";


fn main() {
    textdomain("beat").unwrap();
    bind_textdomain_codeset("beat", "UTF-8").unwrap();

    gtk::init().unwrap();
    let app = BeatApp::new(APP_ID);
    ui::cli::make_cli(&app);

    app.run();
}
