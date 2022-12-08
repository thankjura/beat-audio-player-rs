use gtk::glib;
use gtk::gio::ApplicationCommandLine;
use gtk::prelude::*;
use crate::{BeatApp, BeatWindow};

pub fn on_command_line(_app: &BeatWindow, _acl: &ApplicationCommandLine) -> i32 {
    println!("On command");
    0
}

pub fn make_cli(app: &BeatApp) {
    app.add_main_option(
        "append",
        glib::Char::from(b'a'),
        glib::OptionFlags::NONE,
        glib::OptionArg::None,
        "Append to current playlist instead of create new",
        None
    );

    app.set_option_context_parameter_string(Some("files"));

    app.connect_command_line(move |app, _acl| {
        if let None = app.window() {
            app.activate();
        };
        println!("command");
        0
    });
}