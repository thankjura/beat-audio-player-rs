use std::rc::Rc;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use crate::queue::QueueManager;
use crate::ui::window::notebook::BeatNotebook;

#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/ui/window.ui")]
pub struct BeatWindowImp {
    #[template_child]
    pub header: TemplateChild<gtk::HeaderBar>,

    #[template_child(id = "body")]
    pub body: TemplateChild<gtk::Box>,

    #[template_child(id = "footer")]
    pub footer: TemplateChild<gtk::Box>,

    #[template_child(id = "notebook")]
    pub notebook: TemplateChild<BeatNotebook>,

    //pub actions: RefCell<Vec<SimpleAction>>,
    pub queue_manager: Rc<QueueManager>,
}

#[glib::object_subclass]
impl ObjectSubclass for BeatWindowImp {
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

impl ObjectImpl for BeatWindowImp {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        let nb = Rc::new(self.notebook.get());
        self.queue_manager.set_notebook(nb);
    }

    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}


impl WidgetImpl for BeatWindowImp {}

impl WindowImpl for BeatWindowImp {}

impl ApplicationWindowImpl for BeatWindowImp {}