//! Desktop integrations that control the player from outside the main window:
//! the MPRIS D-Bus interface (media keys, `playerctl`, shell widgets) and the
//! StatusNotifierItem tray icon.

mod mpris;
mod tray;

use crate::app::BeatApp;
use crate::structs::action::Action;
use crate::utils::meta;
use gstreamer::State;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use mpris::Mpris;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;
use tray::Tray;

/// Step used by the tray icon scroll wheel to change the volume.
const VOLUME_STEP: f64 = 0.05;

/// A request coming from MPRIS or from the tray icon. Commands are always
/// handled on the GTK main thread.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    PlayPause,
    Play,
    Pause,
    Stop,
    Next,
    Prev,
    /// Relative seek, in microseconds.
    SeekBy(i64),
    /// Absolute seek to the given second of the track with the given MPRIS id.
    SeekTo(String, u64),
    SetVolume(f64),
    /// Changes the volume by a number of `VOLUME_STEP`s.
    StepVolume(i32),
    SetShuffle(bool),
    SetRepeat(bool),
    Raise,
    ToggleWindow,
    SetCloseToTray(bool),
    TrayAvailable(bool),
    Quit,
}

/// Names accepted by `beat --action`, as used by the .desktop file actions.
pub const CLI_ACTIONS: [&str; 6] = ["play", "pause", "play-pause", "stop", "next", "prev"];

impl Command {
    pub fn from_cli_action(name: &str) -> Option<Self> {
        match name {
            "play" => Some(Self::Play),
            "pause" => Some(Self::Pause),
            "play-pause" => Some(Self::PlayPause),
            "stop" => Some(Self::Stop),
            "next" => Some(Self::Next),
            "prev" => Some(Self::Prev),
            _ => None,
        }
    }
}

/// What is currently loaded into the player, as exposed over MPRIS and in the
/// tray tooltip.
#[derive(Debug, Clone, PartialEq)]
pub struct NowPlaying {
    /// MPRIS track id (a D-Bus object path).
    pub id: String,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub filepath: String,
    pub length: Option<u64>,
    pub art: Option<PathBuf>,
}

impl NowPlaying {
    /// "Artist — Title", or just the title when the artist is unknown.
    pub fn display_name(&self) -> String {
        match &self.artist {
            Some(artist) => format!("{artist} — {}", self.title),
            None => self.title.clone(),
        }
    }
}

fn track_id(tab_idx: u32, track_idx: u32) -> String {
    format!("/ru/slie/beat/track/t{tab_idx}_{track_idx}")
}

pub struct Remote {
    app: glib::WeakRef<BeatApp>,
    mpris: RefCell<Option<Rc<Mpris>>>,
    tray: RefCell<Option<Tray>>,
    tray_available: Cell<bool>,
    state: Cell<State>,
    current: RefCell<Option<(u32, u32)>>,
    now_playing: RefCell<Option<NowPlaying>>,
    /// Duration reported by the player, keyed by (tab, track). It can arrive
    /// before the state change that makes the track current.
    duration: Cell<Option<((u32, u32), u64)>>,
}

impl Remote {
    pub fn new(app: &BeatApp) -> Rc<Self> {
        Rc::new(Self {
            app: app.downgrade(),
            mpris: RefCell::new(None),
            tray: RefCell::new(None),
            tray_available: Cell::new(false),
            state: Cell::new(State::Null),
            current: RefCell::new(None),
            now_playing: RefCell::new(None),
            duration: Cell::new(None),
        })
    }

    /// Registers the MPRIS service and the tray icon, and starts following the
    /// window controls. Must be called once the main window exists.
    pub fn start(self: &Rc<Self>) {
        let Some(app) = self.app.upgrade() else {
            return;
        };
        let Some(window) = app.get_window() else {
            return;
        };
        let win = window.imp();

        let this = Rc::downgrade(self);
        win.button_volume.connect_value_changed(move |_, value| {
            if let Some(this) = this.upgrade() {
                this.with_mpris(|mpris| mpris.set_volume(value));
            }
        });

        let this = Rc::downgrade(self);
        win.button_shuffle.connect_toggled(move |button| {
            if let Some(this) = this.upgrade() {
                let active = button.is_active();
                this.with_mpris(|mpris| mpris.set_shuffle(active));
            }
        });

        let this = Rc::downgrade(self);
        win.button_repeat.connect_toggled(move |button| {
            if let Some(this) = this.upgrade() {
                let active = button.is_active();
                this.with_mpris(|mpris| mpris.set_repeat(active));
            }
        });

        let this = Rc::downgrade(self);
        window.connect_visible_notify(move |window| {
            if let Some(this) = this.upgrade() {
                let visible = window.is_visible();
                this.with_tray(|tray| tray.set_window_visible(visible));
            }
        });

        let dispatch = self.dispatcher();
        let initial = mpris::InitialState {
            volume: win.button_volume.value(),
            shuffle: win.button_shuffle.is_active(),
            repeat: win.button_repeat.is_active(),
        };
        let this = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            match Mpris::new(dispatch, initial).await {
                Ok(mpris) => {
                    if let Some(this) = this.upgrade() {
                        this.mpris.replace(Some(Rc::new(mpris)));
                        this.sync_mpris();
                    }
                }
                Err(err) => eprintln!("Can't register MPRIS service: {err}"),
            }
        });

        let (sender, receiver) = async_channel::unbounded::<Command>();
        let this = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            while let Ok(command) = receiver.recv().await {
                let Some(this) = this.upgrade() else {
                    break;
                };
                this.handle(command);
            }
        });

        let close_to_tray = self.close_to_tray();
        let visible = window.is_visible();
        let this = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            match Tray::spawn(sender, close_to_tray, visible).await {
                Ok(tray) => {
                    if let Some(this) = this.upgrade() {
                        this.tray.replace(Some(tray));
                        this.handle(Command::TrayAvailable(true));
                        this.sync_tray();
                    }
                }
                Err(err) => eprintln!("Tray icon is not available: {err}"),
            }
        });
    }

    fn dispatcher(self: &Rc<Self>) -> Rc<dyn Fn(Command)> {
        let this = Rc::downgrade(self);
        Rc::new(move |command| {
            if let Some(this) = this.upgrade() {
                this.handle(command);
            }
        })
    }

    fn with_mpris(&self, f: impl FnOnce(&Mpris)) {
        if let Some(mpris) = self.mpris.borrow().as_ref() {
            f(mpris);
        }
    }

    fn with_tray(&self, f: impl FnOnce(&Tray)) {
        if let Some(tray) = self.tray.borrow().as_ref() {
            f(tray);
        }
    }

    fn sync_mpris(&self) {
        let state = self.state.get();
        let now_playing = self.now_playing.borrow().clone();
        self.with_mpris(|mpris| {
            mpris.set_now_playing(now_playing.as_ref());
            mpris.set_state(state);
        });
    }

    fn sync_tray(&self) {
        let playing = self.state.get() == State::Playing;
        let name = self
            .now_playing
            .borrow()
            .as_ref()
            .map(NowPlaying::display_name);
        self.with_tray(|tray| tray.set_playback(playing, name));
    }

    fn close_to_tray(&self) -> bool {
        self.app
            .upgrade()
            .and_then(|app| app.imp().settings.borrow().clone())
            .is_none_or(|settings| settings.lock().unwrap().close_to_tray())
    }

    fn apply_hide_on_close(&self) {
        if let Some(window) = self.app.upgrade().and_then(|app| app.get_window()) {
            window.set_hide_on_close(self.tray_available.get() && self.close_to_tray());
        }
    }

    pub fn on_state_changed(&self, track: Option<(u32, u32)>, state: State) {
        self.state.set(state);
        if track != *self.current.borrow() {
            self.current.replace(track);
            self.now_playing.replace(self.load_now_playing());
        }
        self.sync_mpris();
        self.sync_tray();
    }

    pub fn on_duration_changed(&self, tab_idx: u32, track_idx: u32, duration: u64) {
        self.duration.set(Some(((tab_idx, track_idx), duration)));
        if *self.current.borrow() == Some((tab_idx, track_idx)) {
            if let Some(now_playing) = self.now_playing.borrow_mut().as_mut() {
                now_playing.length = Some(duration);
            }
            let now_playing = self.now_playing.borrow().clone();
            self.with_mpris(|mpris| mpris.set_now_playing(now_playing.as_ref()));
        }
    }

    pub fn on_progress(&self, position: u64) {
        self.with_mpris(|mpris| mpris.set_position(position));
    }

    pub fn on_seeked(&self, position: u64) {
        self.with_mpris(|mpris| mpris.seeked(position));
    }

    fn load_now_playing(&self) -> Option<NowPlaying> {
        let (tab_idx, track_idx) = (*self.current.borrow())?;
        let window = self.app.upgrade()?.get_window()?;
        let track = window.imp().notebook.get().get_track(tab_idx, track_idx)?;
        let length = self
            .duration
            .get()
            .filter(|(key, _)| *key == (tab_idx, track_idx))
            .map(|(_, duration)| duration);

        Some(NowPlaying {
            id: track_id(tab_idx, track_idx),
            title: track.display_title().to_string(),
            artist: track.get_by_name("artist").map(str::to_string),
            album: track.get_by_name("album").map(str::to_string),
            filepath: track.filepath().to_string(),
            length,
            art: meta::get_album_picture_path(track.filepath()),
        })
    }

    pub fn handle(&self, command: Command) {
        let Some(app) = self.app.upgrade() else {
            return;
        };
        let Some(window) = app.get_window() else {
            return;
        };
        let Some(player) = app.imp().player.borrow().clone() else {
            return;
        };
        let win = window.imp();
        let emit_action =
            |action: Action| window.emit_by_name::<()>("action", &[&action.get_value()]);

        match command {
            Command::PlayPause => emit_action(Action::Play),
            Command::Play => {
                if !player.is_playing() {
                    emit_action(Action::Play);
                }
            }
            Command::Pause => {
                if player.is_playing() {
                    emit_action(Action::Play);
                }
            }
            Command::Stop => emit_action(Action::Stop),
            Command::Next => emit_action(Action::Next),
            Command::Prev => emit_action(Action::Prev),
            Command::SeekBy(offset_us) => {
                if let Some(position) = player.position() {
                    let target_us = (position as i64 * 1_000_000).saturating_add(offset_us);
                    player.seek_to((target_us.max(0) / 1_000_000) as u64);
                }
            }
            Command::SeekTo(id, seconds) => {
                let is_current = self
                    .now_playing
                    .borrow()
                    .as_ref()
                    .is_some_and(|now_playing| now_playing.id == id);
                if is_current {
                    player.seek_to(seconds);
                }
            }
            Command::SetVolume(volume) => win.button_volume.set_value(volume.clamp(0.0, 1.0)),
            Command::StepVolume(steps) => {
                let volume = win.button_volume.value() + f64::from(steps) * VOLUME_STEP;
                win.button_volume.set_value(volume.clamp(0.0, 1.0));
            }
            Command::SetShuffle(value) => win.button_shuffle.set_active(value),
            Command::SetRepeat(value) => win.button_repeat.set_active(value),
            Command::Raise => window.present(),
            Command::ToggleWindow => {
                if window.is_visible() && window.is_active() && self.tray_available.get() {
                    window.set_visible(false);
                } else {
                    window.present();
                }
            }
            Command::SetCloseToTray(value) => {
                if let Some(settings) = app.imp().settings.borrow().as_ref() {
                    settings.lock().unwrap().set_close_to_tray(value);
                }
                self.apply_hide_on_close();
            }
            Command::TrayAvailable(available) => {
                self.tray_available.set(available);
                self.apply_hide_on_close();
                // Without a tray there is no way back to a hidden window.
                if !available && !window.is_visible() {
                    window.present();
                }
            }
            Command::Quit => app.quit(),
        }
    }
}
