use gtk::gdk::{BUTTON_PRIMARY, BUTTON_SECONDARY};
use gtk::{ColumnView, gdk, gio, ListView, Orientation, PickFlags};
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::BeatWindow;
use crate::ui::BeatNotebook;
use crate::ui::window::notebook::playlist::cols::{make_icon_column, make_text_column};
use crate::ui::window::notebook::playlist::{ColType, PLAY_LIST_COLS};
use crate::ui::window::notebook::playlist::store::PlayListStore;

#[derive(Debug, Default)]
pub struct PlayList {
    _uuid: String,
    scrollbox: gtk::ScrolledWindow,
    store: PlayListStore,
}

fn get_clicked_row(view: &ColumnView, x: f64, y: f64) -> Option<u32> {
    if let Some(picked_widget) = view.pick(x, y, PickFlags::DEFAULT) {
        if let Some(parent) = picked_widget.ancestor(ListView::static_type()) {
            let mut child = parent.first_child().unwrap();
            let mut index = 0;

            if picked_widget.is_ancestor(&child) {
                return Some(index);
            } else {
                while let Some(next_sibling) = child.next_sibling() {
                    index += 1;
                    if picked_widget.is_ancestor(&next_sibling) {
                        return Some(index);
                    }
                    child = next_sibling;
                }
            }
        }
    }

    None
}

impl PlayList {
    pub fn new_with_uuid(uuid: &str) -> Self {
        let uuid = uuid.to_string();
        let store = PlayListStore::new();

        let scrollbox = gtk::ScrolledWindow::new();
        let container = gtk::Box::new(Orientation::Vertical, 0);
        scrollbox.set_child(Some(&container));
        let view = gtk::ColumnView::new(Some(store.selector()));
        view.set_show_row_separators(true);
        view.set_show_column_separators(true);
        view.set_enable_rubberband(true);

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
        container.append(&view);
        //scrollbox.set_child(Some(&view));

        view.connect_activate(move |view, row_index| {
            if let Some(notebook) = view.ancestor(BeatNotebook::static_type()) {
                if let Some(notebook) = notebook.downcast_ref::<BeatNotebook>() {
                    if let Some(page) = notebook.imp().selected_tab_id() {
                        if let Some(track) = notebook.get_track(page, row_index) {
                            notebook.emit_by_name::<()>("track-activated", &[&page, &row_index, &track.filepath()]);
                        }
                    }
                }
            }
        });

        let drag_box = gtk::DropTarget::new(gtk::gdk::FileList::static_type(), gtk::gdk::DragAction::COPY);
        let view_ref = view.downgrade();

        drag_box.connect_drop(move |_drop, value, _x, _y| {
            let files = value.get::<gtk::gdk::FileList>().unwrap();
            let files: Vec<String> = files.files().iter().map(|f| {f.path().unwrap().to_str().unwrap().to_string()}).collect();
            if let Some(win) = view_ref.upgrade().unwrap().ancestor(BeatWindow::static_type()) {
                if let Some(win) = win.downcast_ref::<BeatWindow>() {
                    win.imp().open_path(files, true);
                }
            }
            true
        });
        view.add_controller(&drag_box);

        let clear_selection_box = gtk::GestureClick::builder().button(BUTTON_PRIMARY).build();
        let view_ref = view.downgrade();
        clear_selection_box.connect_pressed(move |_event, n_press, x, y| {
            if n_press != 1 {
                return ();
            }
            let view = view_ref.upgrade().unwrap();
            if let None = get_clicked_row(&view, x, y) {
                view.model().unwrap().unselect_all();
            }
        });

        view.add_controller(&clear_selection_box);

        let context_menu_box = gtk::GestureClick::builder().button(BUTTON_SECONDARY).build();
        let view_ref = view.downgrade();

        let menu_data = gio::Menu::new();
        let menu_model = gio::MenuModel::from(menu_data);
        let menu = gtk::PopoverMenu::builder().menu_model(&menu_model).build();

        container.append(&menu);

        let add_to_queue = gio::MenuItem::new(Some("Add to queue"), None);
        let rm_from_queue = gio::MenuItem::new(Some("Remove from queue"), None);
        let rm_from_playlist = gio::MenuItem::new(Some("Remove from playlist"), None);

        context_menu_box.connect_pressed(move |_event, n_press, x, y| {
            if n_press != 1 {
                return ();
            }

            let mut in_queue = true;

            let view = view_ref.upgrade().unwrap();
            let selection = view.model().unwrap().selection();
            if let Some(row_num) = get_clicked_row(&view, x, y) {
                if !selection.contains(row_num) {
                    view.model().unwrap().select_item(row_num, true);
                }
            }

            if !view.model().unwrap().selection().is_empty() {
                let menu_data = gio::Menu::new();
                if in_queue {
                    menu_data.append_item(&rm_from_queue);
                } else {
                    menu_data.append_item(&add_to_queue);
                }
                menu_data.append_item(&rm_from_playlist);
                let model = gio::MenuModel::from(menu_data);
                menu.set_menu_model(Some(&model));
                menu.set_pointing_to(Some(&gdk::Rectangle::new(x as i32, y as i32, 0, 0)));
                menu.popup();
            }
        });

        view.add_controller(&context_menu_box);

        Self {
            _uuid: uuid,
            scrollbox,
            //view,
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
