mod template;
mod header;
//mod notebook;
mod actions;
mod notebook;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct BeatWindow(ObjectSubclass<template::BeatWindowTemplate>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl BeatWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }
}
