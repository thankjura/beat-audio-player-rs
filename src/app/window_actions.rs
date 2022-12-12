use std::rc::Rc;
use gstreamer::{MessageView, State};
use gstreamer::prelude::ElementExt;
use gtk::prelude::{AdjustmentExt, ObjectExt};
use gtk::subclass::prelude::*;
use crate::app::imp::BeatAppImp;
use crate::BeatWindow;
use crate::ui::BeatNotebook;

impl BeatAppImp {
    pub fn link_actions(&self, window: Rc<BeatWindow>) {
        let player = self.player.clone();

        window.imp().notebook.get().connect("track_activated", false, move |values| {
            let notebook = values[0].get::<&BeatNotebook>().unwrap();
            let tab_id = values[1].get::<u32>().unwrap();
            let track_id = values[2].get::<u32>().unwrap();
            if let Some(track) = notebook.get_track(tab_id, track_id) {
                player.set_uri(track.filepath());
                player.play();
            }
            None
        });

        let player = self.player.clone();

        window.imp().progress.get().connect_value_changed(move |adj| {
            player.set_position(adj.value());
        });

        let player = self.player.clone();
        let bus = player.pipeline.bus().unwrap();
        bus.add_signal_watch();

        bus.connect_message(Some("state-changed"), move |_bus, msg| {
            if let MessageView::StateChanged(value) = msg.view() {
                if let Some(src) = msg.src() {
                    if src != player.pipeline {
                        return;
                    }
                }
                if let State::Playing = value.current() {
                    println!("Start plaing");
                };
            };
        });

    }
}