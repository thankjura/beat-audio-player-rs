use std::sync::Arc;
use gstreamer::State;
use gtk::glib;
use gtk::prelude::RangeExt;
use crate::{BeatWindow, ObjectExt};
use crate::gio::subclass::prelude::*;
use crate::player::BeatPlayer;
use crate::structs::action::Action;
use crate::utils::meta;

enum Msg {
    DurationChanged(u64),
    StateChanged(Option<(u32, u32, String)>, State),
    ProgressChanged(u64, f64),
    TrackCleared(u32, u32),
    RequestNext,
    RequestPrev,
}

pub fn connect(window: &BeatWindow, player: &Arc<BeatPlayer>) {
    let player_weak = player.downgrade();

    window.imp().notebook.get().connect("track-activated", false, move |values| {
        let tab_idx = values[1].get::<u32>().unwrap();
        let track_idx = values[2].get::<u32>().unwrap();
        let filepath = values[3].get::<String>().unwrap();
        let player = player_weak.upgrade().unwrap();
        player.play_ref(tab_idx, track_idx, filepath);
        None
    });

    let player_weak = player.downgrade();
    window.connect("volume-changed", true, move |values| {
        let value = values[1].get::<f64>().unwrap();
        let player = player_weak.upgrade().unwrap();
        player.set_volume(value);
        None
    });

    let player_weak = player.downgrade();
    let handler_id = window.imp().progress.get().connect_value_changed(move |el| {
        let value = el.value();
        let player = player_weak.upgrade().unwrap();
        player.seek_position(value);
    });

    let player_weak = player.downgrade();
    window.connect("action", false, move |values| {
        let player = player_weak.upgrade().unwrap();
        if let Some(action) = Action::from_value(values[1].get::<u8>().unwrap()) {
            match action {
                Action::PLAY => { player.toggle_play(); }
                Action::STOP => { player.stop(); }
                Action::NEXT => { player.next(); }
                Action::PREV => { player.prev(); }
            }
        }
        None
    });

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let sender_ref = sender.clone();
    player.connect("duration-changed", true, move |values| {
        let duration = values[1].get::<u64>().unwrap();
        sender_ref.send(Msg::DurationChanged(duration)).unwrap();
        None
    });

    let sender_ref = sender.clone();
    player.connect("state-changed", true, move |values| {
        let tab_idx = values[1].get::<i32>().unwrap();
        let track_idx = values[2].get::<i32>().unwrap();
        let track_path = values[3].get::<String>().unwrap();
        let state = values[4].get::<State>().unwrap();

        let track_ref;
        if tab_idx >= 0 && track_idx >= 0 {
            track_ref = Some((tab_idx as u32, track_idx as u32, track_path));
        } else {
            track_ref = None
        }
        sender_ref.send(Msg::StateChanged(track_ref, state)).unwrap();

        None
    });

    let sender_ref = sender.clone();
    player.connect("progress-changed", true, move |values| {
        let position = values[1].get::<u64>().unwrap();
        let progress = values[2].get::<f64>().unwrap();
        sender_ref.send(Msg::ProgressChanged(position, progress)).unwrap();
        None
    });

    let sender_ref = sender.clone();
    player.connect("track-cleared", true, move |values| {
        let tab_idx = values[1].get::<u32>().unwrap();
        let track_idx = values[2].get::<u32>().unwrap();
        sender_ref.send(Msg::TrackCleared(tab_idx, track_idx)).unwrap();
        None
    });

    let sender_ref = sender.clone();
    player.connect("query-next", true, move |_values| {
        sender_ref.send(Msg::RequestNext).unwrap();
        None
    });

    let sender_ref = sender.clone();
    player.connect("query-prev", true, move |_values| {
        sender_ref.send(Msg::RequestPrev).unwrap();
        None
    });

    let spectrum =  window.imp().spectrum.imp().downgrade();
    player.imp().connect_spectrum(move |value| {
        let spectrum = spectrum.upgrade().unwrap();
        spectrum.redraw(value);
    });

    let window_ref = window.downgrade();

    receiver.attach(None, move |msg| {
        let window = window_ref.upgrade().unwrap();
        match msg {
            Msg::DurationChanged(duration) => {
                window.imp().update_duration(duration);
            }
            Msg::StateChanged(track_ref, state) => {
                window.imp().set_playing_icon(State::Playing == state);
                window.imp().spectrum.imp().clear();

                if let Some((tab_idx, track_idx, track_path)) = track_ref {
                    if State::Playing == state {
                        if let Some(path) = meta::get_album_picture_path(&track_path) {
                            window.imp().set_cover(Some(path));
                        }
                    }
                    window.imp().notebook.get().set_track_state(tab_idx, track_idx, Some(state));
                }
            }
            Msg::ProgressChanged(position, progress) => {
                let progress_element = &window.imp().progress;
                progress_element.block_signal(&handler_id);
                window.imp().update_progress(position, progress);
                progress_element.unblock_signal(&handler_id);
            }
            Msg::TrackCleared(tab_idx, track_idx) => {
                window.imp().notebook.get().set_track_state(tab_idx, track_idx, None);
            }
            Msg::RequestNext => {
                window.imp().notebook.get().activate_next();
            }
            Msg::RequestPrev => {
                window.imp().notebook.get().activate_prev();
            }
        }
        glib::Continue(true)
    });
}