#[derive(Debug, Default)]
pub struct Tab {
    label: gtk::Label,
}

impl Tab {
    pub fn new_with_label(name: &str) -> Self {
        let label = gtk::Label::new(Some(name));

        Self {
            label
        }
    }

    pub fn label(&self) -> &gtk::Label {
        &self.label
    }
}
