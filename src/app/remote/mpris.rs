//! `org.mpris.MediaPlayer2` service: lets media keys, `playerctl` and desktop
//! shell widgets see and control playback.

use super::{Command, NowPlaying};
use crate::APP_ID;
use gstreamer::State;
use gtk::glib;
use mpris_server::zbus;
use mpris_server::{LoopStatus, Metadata, PlaybackStatus, Player, Time, TrackId};
use std::rc::Rc;

/// Bus name suffix: the service is published as `org.mpris.MediaPlayer2.beat`.
const BUS_NAME_SUFFIX: &str = "beat";

pub struct InitialState {
    pub volume: f64,
    pub shuffle: bool,
    pub repeat: bool,
}

pub struct Mpris {
    player: Rc<Player>,
    updates: async_channel::Sender<Update>,
}

fn loop_status(repeat: bool) -> LoopStatus {
    if repeat {
        LoopStatus::Playlist
    } else {
        LoopStatus::None
    }
}

fn playback_status(state: State) -> PlaybackStatus {
    match state {
        State::Playing => PlaybackStatus::Playing,
        State::Paused => PlaybackStatus::Paused,
        _ => PlaybackStatus::Stopped,
    }
}

fn file_uri(path: impl AsRef<std::path::Path>) -> Option<String> {
    glib::filename_to_uri(path, None)
        .ok()
        .map(|uri| uri.to_string())
}

fn metadata(now_playing: Option<&NowPlaying>) -> Metadata {
    let Some(now_playing) = now_playing else {
        return Metadata::builder().trackid(TrackId::NO_TRACK).build();
    };

    let mut builder = Metadata::builder().title(&now_playing.title);
    if let Ok(id) = TrackId::try_from(now_playing.id.as_str()) {
        builder = builder.trackid(id);
    }
    if let Some(artist) = &now_playing.artist {
        builder = builder.artist([artist]);
    }
    if let Some(album) = &now_playing.album {
        builder = builder.album(album);
    }
    if let Some(length) = now_playing.length {
        builder = builder.length(Time::from_secs(length as i64));
    }
    if let Some(uri) = file_uri(&now_playing.filepath) {
        builder = builder.url(uri);
    }
    if let Some(uri) = now_playing.art.as_ref().and_then(file_uri) {
        builder = builder.art_url(uri);
    }
    builder.build()
}

impl Mpris {
    pub async fn new(dispatch: Rc<dyn Fn(Command)>, initial: InitialState) -> zbus::Result<Self> {
        let player = Player::builder(BUS_NAME_SUFFIX)
            .identity("Beat")
            .desktop_entry(APP_ID)
            .can_raise(true)
            .can_quit(true)
            .can_play(true)
            .can_pause(true)
            .can_seek(true)
            .can_go_next(true)
            .can_go_previous(true)
            .can_control(true)
            .playback_status(PlaybackStatus::Stopped)
            .metadata(metadata(None))
            .volume(initial.volume)
            .shuffle(initial.shuffle)
            .loop_status(loop_status(initial.repeat))
            .supported_uri_schemes(["file"])
            .build()
            .await?;

        let connect = |command: Command| {
            let dispatch = dispatch.clone();
            move |_: &Player| dispatch(command.clone())
        };
        player.connect_raise(connect(Command::Raise));
        player.connect_quit(connect(Command::Quit));
        player.connect_play_pause(connect(Command::PlayPause));
        player.connect_play(connect(Command::Play));
        player.connect_pause(connect(Command::Pause));
        player.connect_stop(connect(Command::Stop));
        player.connect_next(connect(Command::Next));
        player.connect_previous(connect(Command::Prev));

        let d = dispatch.clone();
        player.connect_seek(move |_, offset| d(Command::SeekBy(offset.as_micros())));
        let d = dispatch.clone();
        player.connect_set_position(move |_, id, position| {
            if !position.is_negative() {
                d(Command::SeekTo(id.to_string(), position.as_secs() as u64));
            }
        });
        let d = dispatch.clone();
        player.connect_set_volume(move |_, volume| d(Command::SetVolume(volume)));
        let d = dispatch.clone();
        player.connect_set_shuffle(move |_, shuffle| d(Command::SetShuffle(shuffle)));
        let d = dispatch;
        player.connect_set_loop_status(move |_, status| {
            d(Command::SetRepeat(status != LoopStatus::None))
        });

        glib::spawn_future_local(player.run());

        let player = Rc::new(player);
        let (updates, receiver) = async_channel::unbounded();
        glib::spawn_future_local(apply_updates(player.clone(), receiver));

        Ok(Self { player, updates })
    }

    fn update(&self, update: Update) {
        let _ = self.updates.try_send(update);
    }

    pub fn set_state(&self, state: State) {
        self.update(Update::Status(playback_status(state)));
    }

    pub fn set_now_playing(&self, now_playing: Option<&NowPlaying>) {
        if now_playing.is_none() {
            self.player.set_position(Time::ZERO);
        }
        self.update(Update::Metadata(metadata(now_playing)));
    }

    pub fn set_position(&self, seconds: u64) {
        // Position changes are not signalled; clients interpolate and query it.
        self.player.set_position(Time::from_secs(seconds as i64));
    }

    pub fn seeked(&self, seconds: u64) {
        let position = Time::from_secs(seconds as i64);
        self.player.set_position(position);
        self.update(Update::Seeked(position));
    }

    pub fn set_volume(&self, volume: f64) {
        self.update(Update::Volume(volume));
    }

    pub fn set_shuffle(&self, shuffle: bool) {
        self.update(Update::Shuffle(shuffle));
    }

    pub fn set_repeat(&self, repeat: bool) {
        self.update(Update::Loop(loop_status(repeat)));
    }
}

/// Property updates are applied one by one, in order: each setter stores the
/// new value and then awaits the `PropertiesChanged` emission, so running them
/// concurrently could let an older value win.
enum Update {
    Status(PlaybackStatus),
    Metadata(Metadata),
    Seeked(Time),
    Volume(f64),
    Shuffle(bool),
    Loop(LoopStatus),
}

async fn apply_updates(player: Rc<Player>, updates: async_channel::Receiver<Update>) {
    while let Ok(update) = updates.recv().await {
        let result = match update {
            Update::Status(status) => player.set_playback_status(status).await,
            Update::Metadata(metadata) => player.set_metadata(metadata).await,
            Update::Seeked(position) => player.seeked(position).await,
            Update::Volume(volume) => player.set_volume(volume).await,
            Update::Shuffle(shuffle) => player.set_shuffle(shuffle).await,
            Update::Loop(status) => player.set_loop_status(status).await,
        };
        if let Err(err) = result {
            eprintln!("MPRIS update failed: {err}");
        }
    }
}
