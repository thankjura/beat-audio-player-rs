use gstreamer::{List, MessageView};
use gstreamer::prelude::ElementExt;
use gtk::glib;
use gtk::prelude::*;
use crate::gio::subclass::prelude::ObjectSubclassExt;
use crate::player::imp::BeatPlayerImp;


impl BeatPlayerImp {
    pub fn watch_bus(&self) {
        let bus = self.pipeline.bus().unwrap();

        let pipeline = self.pipeline.downgrade();
        let obj_ref = self.obj().downgrade();

        bus.connect_message(None, move |_bus, msg| {
            let obj = &obj_ref.upgrade().unwrap();

            match msg.view() {
                MessageView::StateChanged(value) => {
                    if let Some(src) = msg.src() {
                        if src != pipeline.upgrade().unwrap() {
                            return;
                        }
                    }

                    obj.__on_state_changed(value.current());
                },
                MessageView::StreamStart(_state) => {
                    if let Some(src) = msg.src() {
                        let pipeline = pipeline.upgrade().unwrap();
                        if src == pipeline {
                            obj.__on_stream_start();
                        }
                    }
                },
                MessageView::Error(_error) => {
                    obj.__on_error();
                },
                MessageView::Eos(_eos) => {
                    obj.__on_eos();
                },
                _ => (),
            }
        });

        let obj_ref = self.obj().downgrade();

        glib::timeout_add_seconds(1, move || {
            let obj = obj_ref.upgrade().unwrap();
            obj.__tick()
        });
    }

    pub fn connect_spectrum<F>(&self, f: F) where F: Fn(Vec<f32>) -> () + Send + Sync + 'static {
        let bus = self.pipeline.bus().unwrap();

        bus.connect_message(Some("element"), move |_bus, message| {
            if let MessageView::Element(element) = message.view() {
                if let Some(element) = element.structure() {
                    if element.name() != "spectrum" {
                        return;
                    }

                    if let Ok(value) = element.get::<List>("magnitude") {
                        let value: Vec<f32> = value.iter().map(|v| { v.get::<f32>().unwrap() }).collect();
                        f(value);
                    }
                }
            }
        });
    }
}