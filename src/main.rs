extern crate core;

mod app;
mod config;
mod player;
mod structs;
mod ui;
mod utils;

use crate::app::BeatApp;
use crate::ui::BeatWindow;
use config::{GETTEXT_PACKAGE, LOCALEDIR};
use gettextrs::*;
use gtk::gio;
use gtk::prelude::*;

const APP_ID: &str = "ru.slie.beat";

#[cfg(not(debug_assertions))]
fn load_resources() {
    use config::LOCALEDIR;
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/beat.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);
}

#[cfg(debug_assertions)]
fn load_resources() {
    gio::resources_register_include!("beat.gresource").expect(&gettext("Could not load resources"));
}

fn main() {
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    load_resources();

    //gtk::init().unwrap();
    let app = BeatApp::new(APP_ID);
    ui::cli::make_cli(&app);

    std::process::exit(app.run());
}
