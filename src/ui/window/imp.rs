use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use crate::ui::header::BeatHeader;


#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/ui/window.ui")]
pub struct BeatWindow {
    #[template_child]
    pub header: TemplateChild<BeatHeader>,
}


#[glib::object_subclass]
impl ObjectSubclass for BeatWindow {
    const NAME: &'static str = "BeatWindow";
    type Type = super::BeatWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BeatWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        // // Connect to "clicked" signal of `button`
        // self.button.connect_clicked(move |button| {
        //     // Set the label to "Hello World!" after the button has been clicked on
        //     button.set_label("Hello World!");
        // });
    }
}


impl WidgetImpl for BeatWindow {}

impl WindowImpl for BeatWindow {}

impl ApplicationWindowImpl for BeatWindow {}