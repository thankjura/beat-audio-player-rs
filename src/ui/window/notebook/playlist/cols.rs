use std::cell::Ref;
use std::fs;
use std::path::{Path, PathBuf};
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
        if TrackState::Playing == r.state() {
            let path = Path::new("/home/jura/Development/rust/beat-autio-player/resources/icons/play.svg");
            println!("{:?}", fs::canonicalize(path));
            child.set_file(Some("/home/jura/Development/rust/beat-autio-player/resources/icons/play.svg"));
        }
    });

    (col_factory, col)
}

pub fn make_text_column(key: &str, name: &str, resizable: bool) -> (gtk::SignalListItemFactory, gtk::ColumnViewColumn) {
    let col_factory = gtk::SignalListItemFactory::new();
    let col = gtk::ColumnViewColumn::new(Some(name), Some(&col_factory));
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
