use crate::app::remote::Remote;
use crate::gio::subclass::prelude::*;
use crate::player::BeatPlayer;
use crate::structs::action::Action;
use crate::utils::meta;
use crate::utils::settings::BeatSettings;
use crate::{BeatWindow, ObjectExt};
use async_channel::unbounded;
use gstreamer::State;
use gtk::glib;
use gtk::prelude::RangeExt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

enum Msg {
    DurationChanged(u32, u32, u64),
    StateChanged(Option<(u32, u32, String)>, State),
    ProgressChanged(u64, f64),
    TrackCleared(u32, u32),
    RequestNext,
    RequestPrev,
    QueueChanged(u32, u32, u32),
    TabChanged(u32, String),
    Seeked(u64),
}

fn send_message(sender: &async_channel::Sender<Msg>, message: Msg) {
    // The channel is unbounded, so sending never needs to block the GTK main loop.
    let _ = sender.try_send(message);
}

pub fn connect(
    window: &BeatWindow,
    player: &Arc<BeatPlayer>,
    settings: &Arc<Mutex<BeatSettings>>,
    remote: &Rc<Remote>,
) {
    let player_weak = player.downgrade();

    window
        .imp()
        .notebook
        .get()
        .connect("track-activated", false, move |values| {
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
    let handler_id = window
        .imp()
        .progress
        .get()
        .connect_value_changed(move |el| {
            let value = el.value();
            let player = player_weak.upgrade().unwrap();
            player.seek_position(value);
        });

    let player_weak = player.downgrade();
    window.connect("action", false, move |values| {
        let player = player_weak.upgrade().unwrap();
        if let Some(action) = Action::from_value(values[1].get::<u8>().unwrap()) {
            match action {
                Action::Play => {
                    player.toggle_play();
                }
                Action::Stop => {
                    player.stop();
                }
                Action::Next => {
                    player.next();
                }
                Action::Prev => {
                    player.prev();
                }
            }
        }
        None
    });

    let player_weak = player.downgrade();
    let settings_ref = Arc::clone(settings);
    window
        .imp()
        .notebook
        .connect("tab-removed", false, move |values| {
            let player = player_weak.upgrade().unwrap();
            let mut settings = settings_ref.lock().unwrap();
            let tab_idx = values[1].get::<u32>().unwrap();
            let tab_uuid = values[2].get::<String>().unwrap();
            player.rm_tab_from_queue(tab_idx);
            settings.drop_playlist(&tab_uuid);
            None
        });

    let player_weak = player.downgrade();
    window
        .imp()
        .notebook
        .connect("queue-add", false, move |values| {
            let player = player_weak.upgrade().unwrap();
            let tab_idx = values[1].get::<u32>().unwrap();
            let track_idx = values[2].get::<u32>().unwrap();
            let track_path = values[3].get::<String>().unwrap();
            player.add_to_queue(tab_idx, track_idx, track_path);
            None
        });

    let player_weak = player.downgrade();
    window
        .imp()
        .notebook
        .connect("queue-rm", false, move |values| {
            let player = player_weak.upgrade().unwrap();
            let tab_idx = values[1].get::<u32>().unwrap();
            let track_idx = values[2].get::<u32>().unwrap();
            player.rm_from_queue(tab_idx, track_idx);
            None
        });

    let (sender, receiver) = unbounded();

    let sender_ref = sender.clone();
    window
        .imp()
        .notebook
        .connect("tab-changed", false, move |values| {
            let tab_idx = values[1].get::<u32>().unwrap();
            let tab_uuid = values[2].get::<String>().unwrap();
            send_message(&sender_ref, Msg::TabChanged(tab_idx, tab_uuid));
            None
        });

    let sender_ref = sender.clone();
    player.connect("duration-changed", true, move |values| {
        let tab_idx = values[1].get::<u32>().unwrap();
        let track_idx = values[2].get::<u32>().unwrap();
        let duration = values[3].get::<u64>().unwrap();
        send_message(
            &sender_ref,
            Msg::DurationChanged(tab_idx, track_idx, duration),
        );
        None
    });

    let sender_ref = sender.clone();
    player.connect("state-changed", true, move |values| {
        let tab_idx = values[1].get::<i32>().unwrap();
        let track_idx = values[2].get::<i32>().unwrap();
        let track_path = values[3].get::<String>().unwrap();
        let state = values[4].get::<State>().unwrap();

        let track_ref = if tab_idx >= 0 && track_idx >= 0 {
            Some((tab_idx as u32, track_idx as u32, track_path))
        } else {
            None
        };

        send_message(&sender_ref, Msg::StateChanged(track_ref, state));

        None
    });

    let sender_ref = sender.clone();
    player.connect("progress-changed", true, move |values| {
        let position = values[1].get::<u64>().unwrap();
        let progress = values[2].get::<f64>().unwrap();
        send_message(&sender_ref, Msg::ProgressChanged(position, progress));
        None
    });

    let sender_ref = sender.clone();
    player.connect("track-cleared", true, move |values| {
        let tab_idx = values[1].get::<u32>().unwrap();
        let track_idx = values[2].get::<u32>().unwrap();
        send_message(&sender_ref, Msg::TrackCleared(tab_idx, track_idx));
        None
    });

    let sender_ref = sender.clone();
    player.connect("seeked", true, move |values| {
        let position = values[1].get::<u64>().unwrap();
        send_message(&sender_ref, Msg::Seeked(position));
        None
    });

    let sender_ref = sender.clone();
    player.connect("query-next", true, move |_values| {
        send_message(&sender_ref, Msg::RequestNext);
        None
    });

    let sender_ref = sender.clone();
    player.connect("query-prev", true, move |_values| {
        send_message(&sender_ref, Msg::RequestPrev);
        None
    });

    let sender_ref = sender;
    player.connect("queue-changed", true, move |values| {
        let tab_idx = values[1].get::<u32>().unwrap();
        let track_idx = values[2].get::<u32>().unwrap();
        let position = values[3].get::<u32>().unwrap();
        send_message(&sender_ref, Msg::QueueChanged(tab_idx, track_idx, position));
        None
    });

    let spectrum = window.imp().spectrum.imp().downgrade();
    player.imp().connect_spectrum(move |value| {
        if let Some(spectrum) = spectrum.upgrade() {
            spectrum.redraw(value);
        }
    });

    let window_ref = window.downgrade();
    let settings_ref = Arc::clone(settings);
    let remote = Rc::downgrade(remote);

    glib::MainContext::default().spawn_local(async move {
        while let Ok(msg) = receiver.recv().await {
            let Some(window) = window_ref.upgrade() else {
                break;
            };
            if let Some(remote) = remote.upgrade() {
                match &msg {
                    Msg::StateChanged(track_ref, state) => remote.on_state_changed(
                        track_ref.as_ref().map(|(tab, track, _)| (*tab, *track)),
                        *state,
                    ),
                    Msg::DurationChanged(tab_idx, track_idx, duration) => {
                        remote.on_duration_changed(*tab_idx, *track_idx, *duration)
                    }
                    Msg::ProgressChanged(position, _) => remote.on_progress(*position),
                    Msg::Seeked(position) => remote.on_seeked(*position),
                    _ => {}
                }
            }
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
                        window.imp().notebook.get().set_track_state(
                            tab_idx,
                            track_idx,
                            Some(state),
                        );
                    }
                }
                Msg::ProgressChanged(position, progress) => {
                    let progress_element = &window.imp().progress;
                    progress_element.block_signal(&handler_id);
                    window.imp().update_progress(position, progress);
                    progress_element.unblock_signal(&handler_id);
                }
                Msg::TrackCleared(tab_idx, track_idx) => {
                    window
                        .imp()
                        .notebook
                        .get()
                        .set_track_state(tab_idx, track_idx, None);
                }
                Msg::RequestNext => {
                    window.imp().notebook.get().activate_next();
                }
                Msg::RequestPrev => {
                    window.imp().notebook.get().activate_prev();
                }
                Msg::QueueChanged(tab_idx, track_idx, position) => {
                    window
                        .imp()
                        .notebook
                        .get()
                        .set_track_position(tab_idx, track_idx, position);
                }
                Msg::Seeked(_) => {}
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
        }
    });
}
