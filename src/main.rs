extern crate core;

mod ui;
mod player;
mod app;
mod structs;
mod utils;

use gettextrs::*;
use gtk::prelude::*;
use gtk::gio;
use crate::ui::BeatWindow;
use crate::app::BeatApp;

const APP_ID: &str = "ru.slie.beat";


fn main() {
    textdomain("beat").unwrap();
    bind_textdomain_codeset("beat", "UTF-8").unwrap();

    gio::resources_register_include!("beat.gresource")
        .expect(&gettext("Failed to register resources."));

    //gtk::init().unwrap();
    let app = BeatApp::new(APP_ID);
    ui::cli::make_cli(&app);

    app.run();
}
