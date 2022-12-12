use std::borrow::Borrow;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{gdk, gio, glib, ScrolledWindow};
use crate::ui::window::notebook::playlist::{PlayList, Track};

#[derive(Debug)]
pub struct Tab {
    widget: gtk::Box,
    event_box: gtk::GestureClick,
    label: gtk::Label,
    menu: Rc<gtk::PopoverMenu>,
    playlist: PlayList,
}

impl Tab {
    pub fn new(name: &str) -> Self {
        let label = gtk::Label::new(Some(name));
        let playlist = PlayList::new_with_uuid(&uuid::Uuid::new_v4().to_string());
        let widget = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let event_box = gtk::GestureClick::builder()
            .button(gdk::BUTTON_SECONDARY)
            .propagation_phase(gtk::PropagationPhase::Capture)
            .build();
        label.add_controller(&event_box);
        widget.append(&label);

        let menu_data = gio::Menu::new();

        let menu_item_rename = gio::MenuItem::new(Some("Rename"), None);
        let menu_item_close = gio::MenuItem::new(Some("Close"), Some("tab.close"));
        menu_data.append_item(&menu_item_rename);
        menu_data.append_item(&menu_item_close);

        let menu_model = gio::MenuModel::from(menu_data);

        let menu = gtk::PopoverMenu::builder().menu_model(&menu_model).build();

        let label = gtk::Label::new(Some("Rename"));

        widget.append(&menu);
        let menu = Rc::new(menu);

        event_box.connect_released(glib::clone!(@weak menu =>
            move |gesture, _count, _x, _y| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                menu.popup();
            }
        ));

        Self {
            widget,
            event_box,
            label,
            menu,
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
        self.playlist.store().add_row(track);
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
}