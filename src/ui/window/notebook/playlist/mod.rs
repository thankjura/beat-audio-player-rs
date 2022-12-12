mod widget;
mod store;
mod cell_render;
mod cols;

pub use widget::PlayList;
pub use crate::structs::track::Track;
pub use crate::structs::track::TrackState;

enum ColType {
    Text,
    Icon,
    Duration,
}

struct PlayListCol {
    key: &'static str,
    label: &'static str,
    col_type: ColType
}

const PLAY_LIST_COLS: [PlayListCol; 5] = [
    PlayListCol { key: "", label: "",  col_type: ColType::Icon},
    PlayListCol { key: "artist", label: "Artist",  col_type: ColType::Text},
    PlayListCol { key: "album", label: "Album",  col_type: ColType::Text},
    PlayListCol { key: "title", label: "Title",  col_type: ColType::Text},
    PlayListCol { key: "duration", label: "Duration",  col_type: ColType::Duration},
];

