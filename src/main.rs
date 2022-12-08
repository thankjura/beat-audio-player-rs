mod ui;
mod player;
mod queue;
mod app;

use gtk::prelude::*;
use crate::ui::BeatWindow;
use crate::app::BeatApp;

const APP_ID: &str = "ru.slie.beat";

// fn init(app: &BeatApp) {
//     // let mut beat_app = BeatApp::new();
//     // if let Some(playlist_pos) = beat_app.open_one("/home/jura/Music/test.ogg", true) {
//     //     beat_app.set_position(&playlist_pos);
//     //     beat_app.play();
//     // }
//     //
//     // app.connect_shutdown(move |_| {
//     //     &beat_app.destroy();
//     // });
//
//
//
//     let window = BeatWindow::new(app);
//     window.setup_actions();
//     // window.open_path("/home/jura/Music/Король и Шут/2001 - Как в старой сказке/11. Двухголовый отпрыск.m4a");
//     // window.open_path("/home/jura/Music/test.ogg");
//     // window.imp().notebook.imp().add_tab("second");
//     // window.open_path("/home/jura/Music/Король и Шут/2001 - Как в старой сказке/11. Двухголовый отпрыск.m4a");
//
//     let window = Rc::new(window);
//
//     app.connect_shutdown(glib::clone!(@weak window =>
//         move |_| {
//             window.destroy();
//         }
//     ));
//
//     window.present();
//
// }

fn main() {
    gtk::init().unwrap();

    let app = BeatApp::new(APP_ID);

    //app.connect_activate(init);

    ui::cli::make_cli(&app);

    app.run();
}
