mod cols;
mod store;
mod widget;

pub use crate::structs::track::Track;
pub use widget::PlayList;

enum ColType {
    Text,
    Icon,
    Position,
}

struct PlayListCol {
    key: &'static str,
    label: &'static str,
    col_type: ColType,
    translate: bool,
}

const PLAY_LIST_COLS: [PlayListCol; 6] = [
    PlayListCol {
        key: "",
        label: "",
        col_type: ColType::Icon,
        translate: false,
    },
    PlayListCol {
        key: "artist",
        label: "Artist",
        col_type: ColType::Text,
        translate: true,
    },
    PlayListCol {
        key: "album",
        label: "Album",
        col_type: ColType::Text,
        translate: true,
    },
    PlayListCol {
        key: "title",
        label: "Title",
        col_type: ColType::Text,
        translate: true,
    },
    PlayListCol {
        key: "duration",
        label: "Duration",
        col_type: ColType::Text,
        translate: true,
    },
    PlayListCol {
        key: "position",
        label: "",
        col_type: ColType::Position,
        translate: false,
    },
];
