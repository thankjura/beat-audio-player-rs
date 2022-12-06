mod ui;
mod player;

use gstreamer_player::gst;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Application;
use crate::ui::BeatWindow;

const APP_ID: &str = "ru.slie.beat";

fn init(app: &Application) {
    // let mut beat_app = BeatApp::new();
    // if let Some(playlist_pos) = beat_app.open_one("/home/jura/Music/test.ogg", true) {
    //     beat_app.set_position(&playlist_pos);
    //     beat_app.play();
    // }
    //
    // app.connect_shutdown(move |_| {
    //     &beat_app.destroy();
    // });

    let window = BeatWindow::new(app);
    window.imp().add_tab("New tab");
    //window.show();
    window.present();
}

fn main() {
    gst::init().unwrap();
    gtk::init().unwrap();

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(init);

    app.run();
}
