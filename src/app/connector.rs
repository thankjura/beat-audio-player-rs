use std::sync::{Arc, Mutex};
use gstreamer::State;
use gtk::glib;
use gtk::prelude::RangeExt;
use crate::{BeatWindow, ObjectExt};
use crate::gio::subclass::prelude::*;
use crate::player::BeatPlayer;
use crate::structs::action::Action;
use crate::utils::meta;
use crate::utils::settings::BeatSettings;

enum Msg {
    DurationChanged(u32, u32, u64),
    StateChanged(Option<(u32, u32, String)>, State),
    ProgressChanged(u64, f64),
    TrackCleared(u32, u32),
    RequestNext,
    RequestPrev,
    QueueChanged(u32, u32, u32),
    TabChanged(u32, String),
}

pub fn connect(window: &BeatWindow, player: &Arc<BeatPlayer>, settings: &Arc<Mutex<BeatSettings>>) {
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

    let player_weak = player.downgrade();
    let settings_ref = Arc::clone(settings);
    window.imp().notebook.connect("tab-removed", false, move |values| {
        let player = player_weak.upgrade().unwrap();
        let mut settings = settings_ref.lock().unwrap();
        let tab_idx = values[1].get::<u32>().unwrap();
        let tab_uuid = values[2].get::<String>().unwrap();
        player.rm_tab_from_queue(tab_idx);
        settings.drop_playlist(&tab_uuid);
        None
    });

    let player_weak = player.downgrade();
    window.imp().notebook.connect("queue-add", false, move |values| {
        let player = player_weak.upgrade().unwrap();
        let tab_idx = values[1].get::<u32>().unwrap();
        let track_idx = values[2].get::<u32>().unwrap();
        let track_path = values[3].get::<String>().unwrap();
        player.add_to_queue(tab_idx, track_idx, track_path);
        None
    });

    let player_weak = player.downgrade();
    window.imp().notebook.connect("queue-rm", false, move |values| {
        let player = player_weak.upgrade().unwrap();
        let tab_idx = values[1].get::<u32>().unwrap();
        let track_idx = values[2].get::<u32>().unwrap();
        player.rm_from_queue(tab_idx, track_idx);
        None
    });

    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let sender_ref = sender.clone();
    window.imp().notebook.connect("tab-changed", false, move |values| {
        let tab_idx = values[1].get::<u32>().unwrap();
        let tab_uuid = values[2].get::<String>().unwrap();
        sender_ref.send(Msg::TabChanged(tab_idx, tab_uuid)).unwrap();
        None
    });

    let sender_ref = sender.clone();
    player.connect("duration-changed", true, move |values| {
        let tab_idx = values[1].get::<u32>().unwrap();
        let track_idx = values[2].get::<u32>().unwrap();
        let duration = values[3].get::<u64>().unwrap();
        sender_ref.send(Msg::DurationChanged(tab_idx, track_idx, duration)).unwrap();
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


    let sender_ref = sender.clone();
    player.connect("queue-changed", true, move |values| {
        let tab_idx = values[1].get::<u32>().unwrap();
        let track_idx = values[2].get::<u32>().unwrap();
        let position = values[3].get::<u32>().unwrap();
        sender_ref.send(Msg::QueueChanged(tab_idx, track_idx, position)).unwrap();
        None
    });

    let spectrum =  window.imp().spectrum.imp().downgrade();
    player.imp().connect_spectrum(move |value| {
        let spectrum = spectrum.upgrade().unwrap();
        spectrum.redraw(value);
    });

    let window_ref = window.downgrade();
    let settings_ref = Arc::clone(settings);

    receiver.attach(None, move |msg| {
        let window = window_ref.upgrade().unwrap();
        match msg {
            Msg::DurationChanged(tab_idx, track_idx, duration) => {
                window.imp().update_duration(tab_idx, track_idx, duration);
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
            Msg::QueueChanged(tab_idx, track_idx, position) => {
                window.imp().notebook.get().set_track_position(tab_idx, track_idx, position);
            }
            Msg::TabChanged(tab_idx, tab_uuid) => {
                let mut settings = settings_ref.lock().unwrap();
                let tracks = window.imp().notebook.get().get_tracks(tab_idx);
                let mut tab_name = window.imp().notebook.get().tab_name(tab_idx);
                if tab_name.is_none() {
                    tab_name.replace("unknown".to_string());
                }
                settings.save_playlist(&tab_uuid, &tab_name.unwrap(), tracks)
            }
        }
        glib::Continue(true)
    });
}