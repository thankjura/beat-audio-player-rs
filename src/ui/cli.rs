use crate::app::{Command, CLI_ACTIONS};
use crate::gio::subclass::prelude::ObjectSubclassIsExt;
use crate::BeatApp;
use gettextrs::gettext;
use gtk::gio::ApplicationCommandLine;
use gtk::glib;
use gtk::glib::ExitCode;
use gtk::prelude::*;
use std::ops::ControlFlow;

pub fn on_command_line(app: &BeatApp, command_line: &ApplicationCommandLine) -> ExitCode {
    let options = command_line.options_dict();
    let append = matches!(options.lookup::<bool>("append"), Ok(Some(_)));

    // The name was validated in `check_local_options`.
    let action = options
        .lookup::<String>("action")
        .ok()
        .flatten()
        .and_then(|name| Command::from_cli_action(&name));

    let files: Vec<String> = command_line.arguments()[1..]
        .iter()
        .filter_map(|p| p.to_str().map(str::to_string))
        .collect();

    if let Some(win) = app.get_window() {
        // A bare action (e.g. from the .desktop menu or a media key binding)
        // controls the running player without pulling up a window hidden to tray.
        if action.is_none() || !files.is_empty() {
            win.present();
        }
    } else {
        app.activate();
    }

    if !files.is_empty() {
        app.imp()
            .window
            .borrow()
            .as_ref()
            .unwrap()
            .imp()
            .open_path(files, append);
    }

    if let Some(command) = action {
        app.run_command(command);
    }

    ExitCode::SUCCESS
}

/// Runs in the calling process, before it forwards the command line to the
/// primary instance, so errors are printed in the user's terminal.
fn check_local_options(_app: &BeatApp, options: &glib::VariantDict) -> ControlFlow<ExitCode> {
    if let Ok(Some(name)) = options.lookup::<String>("action") {
        if Command::from_cli_action(&name).is_none() {
            eprintln!(
                "{}: {name}. {}: {}",
                gettext("Unknown action"),
                gettext("Available actions"),
                CLI_ACTIONS.join(", "),
            );
            return ControlFlow::Break(ExitCode::FAILURE);
        }
    }
    ControlFlow::Continue(())
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

    app.add_main_option(
        "action",
        glib::Char::from(0),
        glib::OptionFlags::NONE,
        glib::OptionArg::String,
        &format!(
            "{} ({})",
            gettext("Control playback of the running player"),
            CLI_ACTIONS.join(", ")
        ),
        Some("ACTION"),
    );

    app.set_option_context_parameter_string(Some("files"));
    app.connect_handle_local_options(check_local_options);
    app.connect_command_line(on_command_line);
}
