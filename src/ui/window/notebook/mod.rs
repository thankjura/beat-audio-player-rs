mod tab;
mod template;
mod actions;
mod playlist;

use std::borrow::Cow;
use std::cell::Ref;
use std::ops::Deref;
use gstreamer::glib::VariantTy;
use gtk::glib;
use gtk::glib::{FromVariant, Variant};
use gtk::prelude::ToVariant;
use gtk::subclass::prelude::*;
pub use playlist::Track;
use crate::StaticVariantType;
use crate::ui::window::notebook::playlist::PlayList;
use crate::ui::window::notebook::tab::Tab;

glib::wrapper! {
    pub struct BeatNotebook(ObjectSubclass<template::BeatNotebookTemplate>)
        @extends gtk::Widget;
}

impl BeatNotebook {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }

    pub fn get_track(&self, track_ref: &TrackRef) -> Option<Track> {
        let tab_index = usize::try_from(track_ref.playlist).unwrap();
        if let Some(tab) = self.imp().tabs.borrow().get(tab_index) {
            return tab.playlist().store().get_row(track_ref.index);
        }

        None
    }
}

#[derive(Debug)]
pub struct TrackRef {
    playlist: u32,
    index: u32,
}

impl TrackRef {
    pub fn new(playlist: u32, index: u32) -> Self {
        Self {
            playlist,
            index
        }
    }
}

impl ToVariant for TrackRef {
    fn to_variant(&self) -> Variant {
        (self.playlist, self.index).to_variant()
    }
}

impl StaticVariantType for TrackRef {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        <(u32, u32)>::static_variant_type()
    }
}

impl FromVariant for TrackRef {
    fn from_variant(variant: &Variant) -> Option<Self> {
        let (playlist, index) = variant.get::<(u32, u32)>().expect("The variant needs to be of type (u32, u32).");
        Some(Self {
            playlist,
            index
        })
    }
}