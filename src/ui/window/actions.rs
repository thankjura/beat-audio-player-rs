use gtk::subclass::prelude::*;
use gtk::gio::SimpleAction;
use gtk::prelude::*;
use crate::ui::window::notebook::TrackRef;

impl super::BeatWindow {
    pub fn setup_actions(&self) {
        let action = SimpleAction::new("track_activate", Some(&TrackRef::static_variant_type()));
        let q = self.imp().queue_manager.clone();
        action.connect_activate(move |_action, parameter| {
            let track_ref = parameter.expect("No track received").get::<TrackRef>().expect("Not is TrackRef format");
            q.play(track_ref);
        });
        self.add_action(&action);



        // let close_action = SimpleAction::new("tab.close", None);
        // close_action.connect_activate(|a, b| {
        //     println!("aaaa");
        // });
        // self.add_action(&close_action);
    }
}