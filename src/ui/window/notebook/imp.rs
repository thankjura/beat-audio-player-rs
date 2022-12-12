use std::cell::RefCell;
use std::rc::Rc;
use gtk::subclass::prelude::*;
use gtk::glib;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use crate::ui::window::notebook::tab::Tab;

#[derive(Default, Debug)]
pub struct BeatNotebookImp {
    pub notebook: gtk::Notebook,
    pub tabs: RefCell<Vec<Rc<Tab>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for BeatNotebookImp {
    const NAME: &'static str = "BeatNotebook";
    type Type = super::BeatNotebook;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for BeatNotebookImp {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("track-activated")
                .param_types([u32::static_type(), u32::static_type()])
                .build()]
        });

        //let a = u32::value_type();
        SIGNALS.as_ref()
    }

    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        let obj = self.obj();
        self.notebook.set_parent(&*obj);
    }

    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}


impl WidgetImpl for BeatNotebookImp {}