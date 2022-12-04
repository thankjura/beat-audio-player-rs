use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};


#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/ui/footer.ui")]
pub struct BeatFooter {

}


#[glib::object_subclass]
impl ObjectSubclass for BeatFooter {
    const NAME: &'static str = "BeatFooter";
    type Type = super::BeatFooter;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        // klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BeatFooter {
    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for BeatFooter {}