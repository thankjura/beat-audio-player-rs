use gstreamer::MessageView;
use gstreamer::prelude::ElementExt;
use gtk::glib;
use gtk::prelude::ObjectExt;
use crate::gio::subclass::prelude::ObjectSubclassExt;
use crate::player::imp::BeatPlayerImp;


impl BeatPlayerImp {
    pub fn watch_bus(&self) {
        let bus = self.pipeline.bus().unwrap();

        let pipeline = self.pipeline.downgrade();
        let obj_ref = self.obj().downgrade();

        bus.add_watch(move |_bus, msg| {
            let obj = &obj_ref.upgrade().unwrap();

            match msg.view() {
                MessageView::StateChanged(value) => {
                    if let Some(src) = msg.src() {
                        if src != pipeline.upgrade().unwrap() {
                            return glib::Continue(true);
                        }
                    }

                    obj.__on_state_changed(value.current());
                },
                MessageView::StreamStart(_state) => {
                    if let Some(src) = msg.src() {
                        let pipeline = pipeline.upgrade().unwrap();
                        if src == pipeline {
                            obj.__on_stream_start();
                            //sender_ref.send(BusMsg::StreamStart);
                        }
                    }
                },
                MessageView::Error(error) => {
                    obj.__on_error();
                    //sender_ref.send(BusMsg::Error);
                },
                MessageView::Eos(eos) => {
                    obj.__on_eos();
                    //sender_ref.send(BusMsg::Eos);
                },
                _ => (),
            }
            glib::Continue(true)
        });

        let obj_ref = self.obj().downgrade();

        glib::timeout_add_seconds(1, move || {
            let obj = obj_ref.upgrade().unwrap();
            obj.__tick()

        });
    }
}