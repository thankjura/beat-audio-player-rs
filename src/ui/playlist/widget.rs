use std::cell::Ref;
use gtk::{Inscription, ScrolledWindow};
use gtk::glib::BoxedAnyObject;
use uuid::Uuid;
use gtk::prelude::*;
use crate::ui::playlist::store::{PlayListStore, Row};

#[derive(Debug, Default)]
pub struct PlayList {
    uuid: String,
    scrollbox: gtk::ScrolledWindow,
    view: gtk::ColumnView,
    store: PlayListStore,
}

fn make_column(name: &str, field: &str) -> (gtk::SignalListItemFactory, gtk::ColumnViewColumn) {
    let col_factory = gtk::SignalListItemFactory::new();
    let col = gtk::ColumnViewColumn::new(Some(name), Some(&col_factory));
    col.set_resizable(true);
    col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = gtk::Inscription::new(None);
        item.set_child(Some(&row));
    });

    let field = field.to_string();

    col_factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().unwrap().downcast::<Inscription>().unwrap();
        let entry = item.item().unwrap().downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Row> = entry.borrow();
        let value = r.get_by_name(&field);
        child.set_text(value);
    });

    (col_factory, col)
}

impl PlayList {
    pub fn new_with_uuid(uuid: &str) -> Self {
        let uuid = uuid.to_string();
        let store = PlayListStore::new();
        store.fill_data();
        let scrollbox = gtk::ScrolledWindow::new();
        let view = gtk::ColumnView::new(Some(store.selector()));
        let (_col_factory1, col1) = make_column("Name", "name");
        let (_col_factory2, col2) = make_column("Path", "path");
        view.set_show_row_separators(true);
        view.set_show_column_separators(true);
        view.append_column(&col1);
        view.append_column(&col2);

        scrollbox.set_child(Some(&view));


        Self {
            uuid,
            scrollbox,
            view,
            store
        }
    }

    pub fn new() -> Self {
        let uuid = Uuid::new_v4().to_string();
        PlayList::new_with_uuid(&uuid)
    }

    pub fn scrollbox(&self) -> &ScrolledWindow {
        &self.scrollbox
    }
}
