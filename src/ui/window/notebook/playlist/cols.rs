use std::cell::Ref;
use gettextrs::gettext;
use gstreamer::glib::BoxedAnyObject;
use gtk::prelude::*;
use gtk::{ColumnViewColumn, Inscription, SignalListItemFactory};
use crate::structs::track::{Track, TrackState};


pub fn make_icon_column(_key: &str, name: &str) -> (SignalListItemFactory, ColumnViewColumn) {
    let col_factory = gtk::SignalListItemFactory::new();
    let col = gtk::ColumnViewColumn::new(Some(name), Some(&col_factory));
    col.set_resizable(false);
    col.set_expand(false);
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
        match r.state() {
            TrackState::Playing => {
                child.set_resource(Some("/ru/slie/beat/icons/play.svg"));
            }
            TrackState::Pause => {
                child.set_resource(Some("/ru/slie/beat/icons/pause.svg"));
            }
            TrackState::Active => {
                child.set_resource(Some("/ru/slie/beat/icons/active.svg"));
            }
            TrackState::None => {
                child.set_resource(None);
            }
        }
    });

    (col_factory, col)
}

pub fn make_text_column(key: &str, name: &str, resizable: bool, translate: bool) -> (gtk::SignalListItemFactory, gtk::ColumnViewColumn) {
    let col_factory = gtk::SignalListItemFactory::new();
    let col;
    if translate {
        col = gtk::ColumnViewColumn::new(Some(&gettext(name)), Some(&col_factory));
    } else {
        col = gtk::ColumnViewColumn::new(Some(name), Some(&col_factory));
    }
    col.set_resizable(resizable);
    col.set_expand(true);
    col_factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = gtk::Inscription::new(None);
        item.set_child(Some(&row));
    });

    let field = key.to_string();

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
