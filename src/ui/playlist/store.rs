use gtk::{glib, gio};
use gtk::prelude::*;


#[derive(Debug)]
pub struct PlayListStore {
    store: gio::ListStore,
    selector: gtk::SingleSelection,
}

pub struct Row {
    name: String,
    path: String,
}

impl Row {
    pub fn get_by_name(&self, field: &str) -> Option<&str> {
        match field {
            "name" => {
                Some(&self.name)
            },
            "path" => {
                Some(&self.path)
            },
            _ => {
                None
            }
        }
    }
}

impl PlayListStore {
    pub fn new() -> Self {
        let store = gio::ListStore::new(glib::BoxedAnyObject::static_type());
        let selector = gtk::SingleSelection::new(Some(&store));
        // let col_name = gtk::ColumnViewColumn::new(Some("Name"), None);
        //
        // let columns = vec![
        //     col_name
        // ];


        Self {
            store,
            selector
        }
    }

    pub fn selector(&self) -> &gtk::SingleSelection {
        &self.selector
    }

    pub fn fill_data(&self) {
        (0..10000).for_each(|i| {
            self.store.append(&glib::BoxedAnyObject::new(Row {
                name: format!("col1 {}", i),
                path: format!("col2 {}", i),
            }))
        });
    }
}

impl Default for PlayListStore {
    fn default() -> Self {
        Self::new()
    }
}