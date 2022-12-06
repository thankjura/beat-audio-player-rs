use crate::ui::playlist::PlayList;

#[derive(Debug, Default)]
pub struct Tab {
    label: gtk::Label,
    playlist: PlayList
}

impl Tab {
    pub fn new(name: &str) -> Self {
        let label = gtk::Label::new(Some(name));
        let playlist = PlayList::new_with_uuid(&uuid::Uuid::new_v4().to_string());

        Self {
            label,
            playlist
        }
    }

    pub fn label(&self) -> &gtk::Label {
        &self.label
    }

    pub fn playlist(&self) -> &PlayList {
        &self.playlist
    }
}
