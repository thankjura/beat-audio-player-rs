use std::cell::Ref;
use gtk::{Inscription, ScrolledWindow};
use gtk::glib::BoxedAnyObject;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::ui::BeatNotebook;
use crate::ui::window::notebook::playlist::store::PlayListStore;
use crate::ui::window::notebook::playlist::track::Track;

#[derive(Debug, Default)]
pub struct PlayList {
    uuid: String,
    scrollbox: gtk::ScrolledWindow,
    view: gtk::ColumnView,
    store: PlayListStore,
}

fn make_column(name: &str, field: &str, resizable: bool) -> (gtk::SignalListItemFactory, gtk::ColumnViewColumn) {
    let col_factory = gtk::SignalListItemFactory::new();
    let col = gtk::ColumnViewColumn::new(Some(name), Some(&col_factory));
    col.set_resizable(resizable);
    col.set_expand(true);
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
        let r: Ref<Track> = entry.borrow();
        let value = r.get_by_name(&field);
        child.set_text(value);
    });

    (col_factory, col)
}

impl PlayList {
    pub fn new_with_uuid(uuid: &str) -> Self {
        let uuid = uuid.to_string();
        let store = PlayListStore::new();

        let scrollbox = gtk::ScrolledWindow::new();
        let view = gtk::ColumnView::new(Some(store.selector()));

        let (_col_factory1, col1) = make_column("Title", "title", false);
        let (_col_factory2, col2) = make_column("Filename", "filename", false);
        view.set_show_row_separators(true);
        view.set_show_column_separators(true);
        view.append_column(&col1);
        view.append_column(&col2);

        scrollbox.set_child(Some(&view));

        view.connect_activate(move |view, row_index| {
            if let Some(notebook) = view.ancestor(BeatNotebook::static_type()) {
                let notebook = notebook.downcast::<BeatNotebook>();
                if notebook.is_ok() {
                    let notebook = &notebook.unwrap();
                    if let Some(page) = notebook.imp().selected_tab_id() {
                        notebook.emit_by_name::<()>("track-activated", &[&page, &row_index]);
                    }
                }
            }
        });

        Self {
            uuid,
            scrollbox,
            view,
            store,
        }
    }

    pub fn scrollbox(&self) -> &ScrolledWindow {
        &self.scrollbox
    }

    pub fn store(&self) -> &PlayListStore {
        &self.store
    }

}
