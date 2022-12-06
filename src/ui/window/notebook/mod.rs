mod tab;
mod template;
mod actions;
mod playlist;

use gtk::glib;
pub use playlist::Track;

glib::wrapper! {
    pub struct BeatNotebook(ObjectSubclass<template::BeatNotebookTemplate>)
        @extends gtk::Widget;
}

impl BeatNotebook {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }
}
