use std::rc::Rc;
use std::sync::Arc;
use gstreamer::{MessageView, State};
use gstreamer::prelude::ElementExt;
use gtk::glib;
use gtk::prelude::{AdjustmentExt, ObjectExt};
use gtk::subclass::prelude::*;
use crate::app::imp::BeatAppImp;
use crate::BeatWindow;
use crate::ui::BeatNotebook;

impl BeatAppImp {
    pub fn link_actions(&self, window: Rc<BeatWindow>) {
        let player = self.player.clone();
        let player_weak = Arc::downgrade(&player);

        window.imp().notebook.get().connect("track_activated", false, move |values| {
            let notebook = values[0].get::<&BeatNotebook>().unwrap();
            let tab_id = values[1].get::<u32>().unwrap();
            let track_id = values[2].get::<u32>().unwrap();
            if let Some(track) = notebook.get_track(tab_id, track_id) {
                let player = player_weak.upgrade().unwrap();
                player.set_uri(track.filepath());
                player.play();
            }
            None
        });

        let player_weak = Arc::downgrade(&player);

        window.imp().progress.get().connect_value_changed(move |adj| {
            let player = player_weak.upgrade().unwrap();
            player.set_position(adj.value());
        });

        let player_weak = Arc::downgrade(&player);
        window.connect("stop", false, move |_value| {
            let player = player_weak.upgrade().unwrap();
            player.stop();
            None
        });

        let player_weak = Arc::downgrade(&player);
        window.connect("play", false, move |_value| {
            let player = player_weak.upgrade().unwrap();
            player.toggle_play();
            None
        });

        let player = self.player.clone();
        let bus = player.pipeline.bus().unwrap();
        bus.add_signal_watch();

        let window_ref = window.downgrade();

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        rx.attach(None, move |value| {
            window_ref.upgrade().unwrap().set_playing_icon(value);
            glib::Continue(true)
        });
        bus.connect_message(Some("state-changed"), move |_bus, msg| {
            if let MessageView::StateChanged(value) = msg.view() {
                if let Some(src) = msg.src() {
                    if src != player.pipeline {
                        return;
                    }

                }
                let value = value.current() == State::Playing;
                if let Err(_) = tx.send(value) {
                    println!("error send icon change message");
                }
            };
        });
        //
        // bus.connect_message(Some("error"), move |_bus, msg| {
        //     if let MessageView::Error(value) = msg.view() {
        //         println!("Error son stream: {:#?}", value);
        //     };
        // });

    }
}