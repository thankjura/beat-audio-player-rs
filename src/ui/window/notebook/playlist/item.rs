use crate::structs::track::Track;
use gstreamer::State;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

pub const PROP_ICON_NAME: &str = "icon-name";
pub const PROP_DURATION: &str = "duration";
pub const PROP_QUEUE_POS: &str = "queue-pos";

mod imp {
    use super::*;
    use std::cell::OnceCell;
    use std::sync::OnceLock;

    #[derive(Default)]
    pub struct TrackItem {
        pub track: OnceCell<Track>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TrackItem {
        const NAME: &'static str = "BeatTrackItem";
        type Type = super::TrackItem;
    }

    impl ObjectImpl for TrackItem {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
            PROPERTIES.get_or_init(|| {
                vec![
                    glib::ParamSpecString::builder(PROP_ICON_NAME)
                        .read_only()
                        .build(),
                    glib::ParamSpecString::builder(PROP_DURATION)
                        .read_only()
                        .build(),
                    glib::ParamSpecString::builder(PROP_QUEUE_POS)
                        .read_only()
                        .build(),
                ]
            })
        }

        fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            let track = self.track.get().expect("TrackItem must hold a track");
            match pspec.name() {
                PROP_ICON_NAME => state_icon_name(track.state()).to_value(),
                PROP_DURATION => track.duration().to_value(),
                PROP_QUEUE_POS => track.queue_pos().to_value(),
                name => unreachable!("unknown TrackItem property {name}"),
            }
        }
    }
}

glib::wrapper! {
    /// Playlist row model. Mutable fields are exposed as GObject properties so
    /// the ColumnView cells update in place through bindings, without replacing
    /// the item in the store (which would reset focus and scroll position).
    pub struct TrackItem(ObjectSubclass<imp::TrackItem>);
}

fn state_icon_name(state: Option<State>) -> Option<&'static str> {
    match state? {
        State::Playing => Some("beat-play-symbolic"),
        State::Paused => Some("beat-pause-symbolic"),
        _ => Some("beat-active-symbolic"),
    }
}

impl TrackItem {
    pub fn new(track: Track) -> Self {
        let item: Self = glib::Object::new();
        let _ = item.imp().track.set(track);
        item
    }

    pub fn track(&self) -> &Track {
        self.imp().track.get().expect("TrackItem must hold a track")
    }

    pub fn set_state(&self, state: Option<State>) {
        self.track().set_state(state);
        self.notify(PROP_ICON_NAME);
    }

    pub fn set_duration(&self, duration: u64) {
        self.track().set_duration(duration);
        self.notify(PROP_DURATION);
    }

    pub fn set_queue_pos(&self, position: u32) {
        self.track().set_queue_pos(position);
        self.notify(PROP_QUEUE_POS);
    }
}
