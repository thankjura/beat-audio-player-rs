mod cols;
mod store;
mod widget;

pub use crate::structs::track::Track;
use gettextrs::gettext;
pub use widget::PlayList;

enum ColType {
    Text,
    Icon,
    Position,
}

struct PlayListCol {
    key: &'static str,
    label: String,
    col_type: ColType,
}

fn get_cols() -> Vec<PlayListCol> {
    vec![
        PlayListCol {
            key: "",
            label: String::new(),
            col_type: ColType::Icon,
        },
        PlayListCol {
            key: "artist",
            label: gettext("Artist"),
            col_type: ColType::Text,
        },
        PlayListCol {
            key: "album",
            label: gettext("Album"),
            col_type: ColType::Text,
        },
        PlayListCol {
            key: "title",
            label: gettext("Title"),
            col_type: ColType::Text,
        },
        PlayListCol {
            key: "duration",
            label: gettext("Duration"),
            col_type: ColType::Text,
        },
        PlayListCol {
            key: "position",
            label: String::new(),
            col_type: ColType::Position,
        },
    ]
}
