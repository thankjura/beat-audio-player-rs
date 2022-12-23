use crate::ui::window::notebook::playlist::cols::{
    make_icon_column, make_position_column, make_text_column,
};
use crate::ui::window::notebook::playlist::store::{get_track, PlayListStore};
use crate::ui::window::notebook::playlist::{ColType, PLAY_LIST_COLS};
use crate::ui::BeatNotebook;
use crate::BeatWindow;
use gettextrs::gettext;
use gtk::gdk::{BUTTON_PRIMARY, BUTTON_SECONDARY};
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gdk, gio, ListView, Orientation, PickFlags};

#[derive(Debug)]
pub struct PlayList {
    uuid: String,
    container: gtk::Box,
    store: PlayListStore,
    view: gtk::ColumnView,
}

fn get_clicked_row(view: &gtk::ColumnView, x: f64, y: f64) -> Option<u32> {
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
        container.append(&scrollbox);
        let view = gtk::ColumnView::new(Some(store.selector()));
        view.set_show_row_separators(true);
        view.set_show_column_separators(true);
        view.set_enable_rubberband(true);
        view.set_vexpand(true);

        for col in PLAY_LIST_COLS {
            match col.col_type {
                ColType::Text => {
                    let (_factory, column) =
                        make_text_column(&col.key, &col.label, true, col.translate);
                    view.append_column(&column);
                }
                ColType::Icon => {
                    let (_factory, column) = make_icon_column(&col.key, &col.label);
                    view.append_column(&column);
                }
                ColType::Position => {
                    let (_factory, column) = make_position_column(&col.key, &col.label);
                    view.append_column(&column);
                }
            }
        }
        //container.append(&view);
        scrollbox.set_child(Some(&view));

        view.connect_activate(move |view, row_index| {
            if let Some(notebook) = view.ancestor(BeatNotebook::static_type()) {
                if let Some(notebook) = notebook.downcast_ref::<BeatNotebook>() {
                    if let Some(page) = notebook.imp().selected_tab_id() {
                        if let Some(track) = notebook.get_track(page, row_index) {
                            notebook.emit_by_name::<()>(
                                "track-activated",
                                &[&page, &row_index, &track.filepath()],
                            );
                        }
                    }
                }
            }
        });

        let drag_box = gtk::DropTarget::new(
            gtk::gdk::FileList::static_type(),
            gtk::gdk::DragAction::COPY,
        );
        let view_ref = view.downgrade();

        drag_box.connect_drop(move |_drop, value, _x, _y| {
            let files = value.get::<gtk::gdk::FileList>().unwrap();
            let files: Vec<String> = files
                .files()
                .iter()
                .map(|f| f.path().unwrap().to_str().unwrap().to_string())
                .collect();
            if let Some(win) = view_ref
                .upgrade()
                .unwrap()
                .ancestor(BeatWindow::static_type())
            {
                if let Some(win) = win.downcast_ref::<BeatWindow>() {
                    win.imp().open_path(files, true);
                }
            }
            true
        });
        view.add_controller(&drag_box);

        let clear_selection_box = gtk::GestureClick::builder().button(BUTTON_PRIMARY).build();
        let view_ref = view.downgrade();
        let store_ref = store.store().downgrade();

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

        let context_menu_box = gtk::GestureClick::builder()
            .button(BUTTON_SECONDARY)
            .build();
        let view_ref = view.downgrade();

        let menu_data = gio::Menu::new();
        let menu_model = gio::MenuModel::from(menu_data);
        let menu = gtk::PopoverMenu::builder().menu_model(&menu_model).build();

        container.append(&menu);

        let add_to_queue =
            gio::MenuItem::new(Some(&gettext("Add to queue")), Some("playlist.queue-add"));
        let rm_from_queue = gio::MenuItem::new(
            Some(&gettext("Remove from queue")),
            Some("playlist.queue-rm"),
        );
        let rm_from_playlist = gio::MenuItem::new(
            Some(&gettext("Remove from playlist")),
            Some("playlist.playlist-rm"),
        );

        context_menu_box.connect_pressed(move |_event, n_press, x, y| {
            if n_press != 1 {
                return ();
            }

            let view = view_ref.upgrade().unwrap();
            let selection = view.model().unwrap().selection();
            if let Some(row_num) = get_clicked_row(&view, x, y) {
                if !selection.contains(row_num) {
                    view.model().unwrap().select_item(row_num, true);
                }
            }

            let selection = view.model().unwrap().selection();
            if !selection.is_empty() {
                let mut show_remove = false;
                let mut show_add = false;

                let store = &store_ref.upgrade().unwrap();
                for i in 0..selection.size() {
                    let position = selection.nth(i as u32);
                    if let Some(item) = get_track(store, position) {
                        if item.queue_pos().is_some() {
                            show_remove = true;
                        } else {
                            show_add = true;
                        }

                        if show_add && show_remove {
                            break;
                        }
                    }
                }

                let menu_data = gio::Menu::new();

                if show_remove {
                    menu_data.append_item(&rm_from_queue);
                }

                if show_add {
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

        // Actions
        let action_group = gio::SimpleActionGroup::new();

        let add_to_queue = gio::SimpleAction::new("queue-add", None);
        let rm_from_queue = gio::SimpleAction::new("queue-rm", None);
        let rm_from_playlist = gio::SimpleAction::new("playlist-rm", None);

        action_group.add_action(&add_to_queue);
        action_group.add_action(&rm_from_queue);
        action_group.add_action(&rm_from_playlist);
        container.insert_action_group("playlist", Some(&action_group));

        let view_ref = view.downgrade();

        add_to_queue.connect_activate(move |_action, _value| {
            let view = view_ref.upgrade().unwrap();
            let notebook = view.ancestor(BeatNotebook::static_type()).unwrap();
            let notebook = notebook.downcast_ref::<BeatNotebook>().unwrap();
            let tab_idx = notebook.imp().selected_tab_id().unwrap();
            let tab = notebook.imp().selected_tab();

            let selection = tab.playlist().view.model().unwrap().selection();
            let select_count = selection.size() as u32;
            for i in 0..select_count {
                let index = selection.nth(i);
                if let Some(track) = tab.playlist().store().get_track(index) {
                    notebook
                        .emit_by_name::<()>("queue-add", &[&tab_idx, &index, &track.filepath()]);
                }
            }
        });

        let view_ref = view.downgrade();

        rm_from_queue.connect_activate(move |_action, _value| {
            let view = view_ref.upgrade().unwrap();
            let notebook = view.ancestor(BeatNotebook::static_type()).unwrap();
            let notebook = notebook.downcast_ref::<BeatNotebook>().unwrap();
            let tab_idx = notebook.imp().selected_tab_id().unwrap();
            let tab = notebook.imp().selected_tab();

            let selection = tab.playlist().view.model().unwrap().selection();
            let select_count = selection.size() as u32;
            for i in 0..select_count {
                let index = selection.nth(i);
                notebook.emit_by_name::<()>("queue-rm", &[&tab_idx, &index]);
            }
            notebook.emit_by_name::<()>("tab-changed", &[&tab.uuid()]);
        });

        let view_ref = view.downgrade();

        rm_from_playlist.connect_activate(move |_action, _value| {
            let view = view_ref.upgrade().unwrap();
            let notebook = view.ancestor(BeatNotebook::static_type()).unwrap();
            let notebook = notebook.downcast_ref::<BeatNotebook>().unwrap();
            let tab_idx = notebook.imp().selected_tab_id().unwrap();
            let tab = notebook.imp().selected_tab();

            let selection = tab.playlist().view.model().unwrap().selection();
            let select_count = selection.size() as u32;
            for i in 0..select_count {
                let index = selection.nth(i);
                tab.playlist().store.rm_track(index);
                notebook.emit_by_name::<()>("queue-rm", &[&tab_idx, &index]);
            }

            notebook.emit_by_name::<()>("tab-changed", &[&tab_idx, &tab.uuid()]);
        });
        // End actions

        Self {
            uuid,
            container,
            store,
            view,
        }
    }

    pub fn body(&self) -> &gtk::Box {
        &self.container
    }

    pub fn store(&self) -> &PlayListStore {
        &self.store
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }
}
