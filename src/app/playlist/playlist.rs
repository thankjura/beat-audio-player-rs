pub struct PlayListItem {
    path: String
}

pub struct PlayList {
    name: String,
    items: Vec<PlayListItem>,
    current_item: Option<usize>
}

pub struct PlayListPosition {
    playlist_idx: usize,
    item_idx: usize,
}

impl PlayList {
    pub fn new(name: &str) -> Self {
        let items = vec![];
        Self {
            name: name.to_string(),
            items,
            current_item: None
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn add_file(&mut self, path: &str) -> usize {
        self.items.push(PlayListItem::new(path));
        self.items.len() - 1
    }

    pub fn set_item(&mut self, idx: usize) -> Option<&PlayListItem> {
        if self.items.len() > idx {
            self.current_item = Some(idx);
            return self.items.get(idx);
        }

        None
    }

    pub fn get_current(&self) -> Option<&PlayListItem> {
        if let Some(item_idx) = self.current_item {
            return self.items.get(item_idx);
        }

        None
    }
}

impl PlayListItem {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string()
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl PlayListPosition {
    pub fn new(playlist_idx: usize, item_idx: usize) -> Self {
        Self {
            playlist_idx,
            item_idx,
        }
    }

    pub fn playlist_idx(&self) -> usize {
        self.playlist_idx
    }

    pub fn item_idx(&self) -> usize {
        self.item_idx
    }
}