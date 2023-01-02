use crate::gio::subclass::prelude::ObjectSubclassIsExt;
use crate::BeatApp;
use gettextrs::gettext;
use gtk::gio::ApplicationCommandLine;
use gtk::glib;
use gtk::prelude::*;

pub fn on_command_line(app: &BeatApp, command_line: &ApplicationCommandLine) -> i32 {
    if let Some(win) = app.get_window() {
        win.present();
    } else {
        app.activate();
    }

    let options = command_line.options_dict();
    let mut append = false;

    if let Ok(val) = options.lookup::<bool>("append") {
        if val.is_some() {
            append = true;
        }
    }

    let files = &command_line.arguments()[1..];
    if !files.is_empty() {
        let mut paths = vec![];
        for p in files {
            if let Some(p) = p.to_str() {
                paths.push(p.to_string());
            }
        }
        app.imp()
            .window
            .borrow()
            .as_ref()
            .unwrap()
            .imp()
            .open_path(paths, append);
    }

    0
}

pub fn make_cli(app: &BeatApp) {
    app.add_main_option(
        "append",
        glib::Char::from(b'a'),
        glib::OptionFlags::NONE,
        glib::OptionArg::None,
        &gettext("Append to current playlist instead of create new"),
        None,
    );

    app.set_option_context_parameter_string(Some("files"));
    app.connect_command_line(on_command_line);
}
