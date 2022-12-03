use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};


#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/ui/header.ui")]
pub struct BeatHeader {

}


#[glib::object_subclass]
impl ObjectSubclass for BeatHeader {
    const NAME: &'static str = "BeatHeader";
    type Type = super::BeatHeader;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BeatHeader {
    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}


impl WidgetImpl for BeatHeader {}

#[gtk::template_callbacks]
impl BeatHeader {
    #[template_callback]
    fn on_open_files(&self, _button: &gtk::Button) {

    }

    #[template_callback]
    fn on_volume_changed(&self, value: f64) {

    }

    #[template_callback]
    fn on_add_files(&self, _button: &gtk::Button) {

    }

    #[template_callback]
    fn on_stop(&self, _button: &gtk::Button) {

    }

    #[template_callback]
    fn on_play(&self, _button: &gtk::Button) {

    }

    #[template_callback]
    fn on_prev(&self, _button: &gtk::Button) {

    }

    #[template_callback]
    fn on_next(&self, _button: &gtk::Button) {

    }
}