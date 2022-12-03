mod ui;
mod player;

use gstreamer_player::gst;
use gtk::prelude::*;
use gtk::Application;
use crate::player::player::BeatPlayer;

const APP_ID: &str = "ru.slie.beat";

fn init(app: &Application) {
    let player = BeatPlayer::build();
    let path = "/home/jura/Music/test.ogg";
    player.set_uri(path);
    let _ = player.play();

    app.connect_shutdown(move |_| {
        let _ = &player.destroy();
    });

    let window = crate::ui::main::build_ui(app);

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