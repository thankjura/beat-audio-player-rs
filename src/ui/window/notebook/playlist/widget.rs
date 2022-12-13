use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::ui::BeatNotebook;
use crate::ui::window::notebook::playlist::cols::{make_icon_column, make_text_column};
use crate::ui::window::notebook::playlist::{ColType, PLAY_LIST_COLS};
use crate::ui::window::notebook::playlist::store::PlayListStore;

#[derive(Debug, Default)]
pub struct PlayList {
    uuid: String,
    scrollbox: gtk::ScrolledWindow,
    view: gtk::ColumnView,
    store: PlayListStore,
}

impl PlayList {
    pub fn new_with_uuid(uuid: &str) -> Self {
        let uuid = uuid.to_string();
        let store = PlayListStore::new();

        let scrollbox = gtk::ScrolledWindow::new();
        let view = gtk::ColumnView::new(Some(store.selector()));
        view.set_show_row_separators(true);
        view.set_show_column_separators(true);

        for col in PLAY_LIST_COLS {
            match col.col_type {
                ColType::Text => {
                    let (_factory, column) = make_text_column(&col.key, &col.label, true, col.translate);
                    view.append_column(&column);
                }
                ColType::Icon => {
                    let (_factory, column) = make_icon_column(&col.key, &col.label);
                    view.append_column(&column);
                }
                ColType::Duration => {}
            }
        }

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

    pub fn scrollbox(&self) -> &gtk::ScrolledWindow {
        &self.scrollbox
    }

    pub fn store(&self) -> &PlayListStore {
        &self.store
    }

}
