use std::borrow::Borrow;
use gtk::prelude::*;
use gtk::{gdk, gio, glib, ScrolledWindow};
use crate::ui::window::notebook::playlist::{PlayList, Track};

#[derive(Debug)]
pub struct Tab {
    widget: gtk::Box,
    label: gtk::Label,
    playlist: PlayList,
}

impl Tab {
    pub fn new(name: &str, uuid: &str) -> Self {
        let label = gtk::Label::new(Some(name));
        let playlist = PlayList::new_with_uuid(uuid);
        let widget = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let event_box = gtk::GestureClick::builder()
            .button(gdk::BUTTON_SECONDARY)
            .propagation_phase(gtk::PropagationPhase::Capture)
            .build();
        widget.add_controller(&event_box);
        widget.append(&label);

        let menu_data = gio::Menu::new();
        let menu_item_rename = gio::MenuItem::new(Some("Rename"), None);
        let menu_item_close = gio::MenuItem::new(Some("Close"), Some("tab.close"));
        menu_data.append_item(&menu_item_rename);
        menu_data.append_item(&menu_item_close);
        let menu_model = gio::MenuModel::from(menu_data);
        let menu = gtk::PopoverMenu::builder().menu_model(&menu_model).build();
        widget.append(&menu);

        event_box.connect_pressed(glib::clone!(@strong menu =>
            move |gesture, count, _x, _y| {
                if count == 1 {
                    gesture.set_state(gtk::EventSequenceState::Claimed);
                    menu.popup();
                }
            }
        ));

        Self {
            widget,
            label,
            playlist,
        }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.widget
    }

    pub fn scrollbox(&self) -> &ScrolledWindow {
        self.playlist.scrollbox()
    }

    pub fn add_track(&self, track: Track) {
        self.playlist.store().add_track(track);
    }

    pub fn has_tracks(&self) -> bool {
        self.playlist.store().has_tracks()
    }

    pub fn playlist(&self) -> &PlayList {
        return &self.playlist;
    }

    pub fn clear_tab(&self) {
        self.label.set_label("new");
        self.playlist.store().borrow().clear();
    }

    pub fn active_track(&self) -> Option<u32> {
        self.playlist.store().active_track()
    }
}
