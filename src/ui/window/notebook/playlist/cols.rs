use crate::structs::track::Track;
use gstreamer::glib::BoxedAnyObject;
use gstreamer::State;
use gtk::prelude::*;
use gtk::{ColumnViewColumn, Inscription, SignalListItemFactory};
use std::cell::Ref;

pub fn make_icon_column(_key: &str, name: &str) -> ColumnViewColumn {
    let col_factory = SignalListItemFactory::new();
    col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = gtk::Image::new();
        row.set_icon_size(gtk::IconSize::Inherit);
        item.set_child(Some(&row));
    });

    col_factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().unwrap().downcast::<gtk::Image>().unwrap();
        let entry = item.item().unwrap().downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Track> = entry.borrow();
        if let Some(state) = r.state() {
            match state {
                State::Playing => {
                    child.set_icon_name(Some("beat-play-symbolic"));
                }
                State::Paused => {
                    child.set_icon_name(Some("beat-pause-symbolic"));
                }
                _ => {
                    child.set_icon_name(Some("beat-active-symbolic"));
                }
            }
        } else {
            child.set_icon_name(None);
        }
    });

    let col = ColumnViewColumn::new(Some(name), Some(col_factory));
    col.set_resizable(false);
    col.set_expand(false);
    col.set_fixed_width(40);

    col
}

pub fn make_position_column(_key: &str, name: &str) -> ColumnViewColumn {
    let col_factory = SignalListItemFactory::new();

    col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = Inscription::new(None);
        item.set_child(Some(&row));
    });

    col_factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().unwrap().downcast::<Inscription>().unwrap();
        let entry = item.item().unwrap().downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Track> = entry.borrow();
        if let Some(value) = r.queue_pos() {
            child.set_text(Some(&value));
        } else {
            child.set_text(None);
        }
    });

    let col = ColumnViewColumn::new(Some(name), Some(col_factory));
    col.set_resizable(false);
    col.set_expand(false);
    col.set_fixed_width(48);

    col
}

pub fn make_text_column(key: &str, name: &str, resizable: bool) -> ColumnViewColumn {
    let col_factory = SignalListItemFactory::new();

    col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = Inscription::new(None);
        item.set_child(Some(&row));
    });

    let field = key.to_string();

    col_factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let child = item.child().unwrap().downcast::<Inscription>().unwrap();
        let entry = item.item().unwrap().downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Track> = entry.borrow();
        if field == "duration" {
            child.set_text(r.duration().as_deref());
        } else if field == "title" {
            child.set_text(Some(r.display_title()));
        } else {
            let value = r.get_by_name(&field);
            child.set_text(value);
        }
    });

    let col = ColumnViewColumn::new(Some(name), Some(col_factory));
    col.set_resizable(resizable);
    col.set_expand(true);

    col
}
