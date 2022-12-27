mod cols;
mod store;
mod widget;

use gettextrs::gettext;
pub use crate::structs::track::Track;
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
    let mut out = Vec::new();

    out.push(
    PlayListCol {
        key: "",
        label: String::new(),
        col_type: ColType::Icon,
    });
    out.push(PlayListCol {
        key: "artist",
        label: gettext("Artist"),
        col_type: ColType::Text,
    });
    out.push(PlayListCol {
        key: "album",
        label: gettext("Album"),
        col_type: ColType::Text,
    });
    out.push(PlayListCol {
        key: "title",
        label: gettext("Title"),
        col_type: ColType::Text,
    });
    out.push(PlayListCol {
        key: "duration",
        label: gettext("Duration"),
        col_type: ColType::Text,
    });
    out.push(PlayListCol {
        key: "position",
        label: String::new(),
        col_type: ColType::Position,
    });

    out
}