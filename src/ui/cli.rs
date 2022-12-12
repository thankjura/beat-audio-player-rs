use gtk::glib;
use gtk::gio::ApplicationCommandLine;
use gtk::prelude::*;
use crate::BeatApp;

pub fn on_command_line(app: &BeatApp, command_line: &ApplicationCommandLine) -> i32 {
    if let None = app.window() {
        app.activate();
    };

    let options = command_line.options_dict();
    let mut append = false;

    if let Ok(val) = options.lookup::<bool>("append") {
        if let Some(_) = val {
            append = true;
        }
    }

    let files = &command_line.arguments()[1..];
    if !files.is_empty() {
        let mut paths = vec![];
        for p in files {
            if let Some(p) = p.to_str() {
                paths.push(p);
            }
        }
        app.open_path(&paths, append);
    }

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

    app.connect_command_line(on_command_line);
}