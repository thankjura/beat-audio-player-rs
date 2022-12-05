use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use crate::ui::window::tab::Tab;


#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/ui/window.ui")]
pub struct BeatWindow {
    #[template_child]
    pub header: TemplateChild<gtk::HeaderBar>,

    #[template_child(id = "body")]
    pub body: TemplateChild<gtk::Box>,

    #[template_child(id = "footer")]
    pub footer: TemplateChild<gtk::Box>,

    #[template_child(id = "notebook")]
    pub notebook: TemplateChild<gtk::Notebook>,

    pub tabs: Vec<Tab>
}

#[glib::object_subclass]
impl ObjectSubclass for BeatWindow {
    const NAME: &'static str = "BeatWindow";
    type Type = super::BeatWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BeatWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        self.notebook.connect_switch_page(move |widget, tab, idx| {
            println!("Switched tabs");
        });
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