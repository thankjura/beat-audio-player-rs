use gtk::{Application};
use crate::ui::window::BeatWindow;

pub fn build_ui(app: &Application) -> BeatWindow {
    // let adjustment = gtk::Adjustment::builder()
    //     .lower(0.0)
    //     .upper(1.0)
    //     .value(0.0)
    //     .build();
    //
    // let sink_widget = gtk::Scale::builder()
    //     .orientation(Orientation::Horizontal)
    //     .adjustment(&adjustment)
    //     .visible(true)
    //     .build();
    //
    // let window = ApplicationWindow::builder()
    //     .application(app)
    //     .title("My GTK App")
    //     .child(&sink_widget)
    //     .default_width(500)
    //     .default_height(300)
    //     .build();
    let window = BeatWindow::new(app);
    window
}