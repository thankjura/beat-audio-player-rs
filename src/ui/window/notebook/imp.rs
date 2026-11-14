use crate::ui::window::notebook::tab::Tab;
use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::Type;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;

#[derive(Default)]
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
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("track-activated")
                    .param_types([u32::static_type(), u32::static_type(), Type::STRING])
                    .build(),
                Signal::builder("tab-removed")
                    .param_types([u32::static_type(), Type::STRING])
                    .build(),
                Signal::builder("tab-changed")
                    .param_types([u32::static_type(), Type::STRING])
                    .build(),
                Signal::builder("queue-add")
                    .param_types([u32::static_type(), u32::static_type(), Type::STRING])
                    .build(),
                Signal::builder("queue-rm")
                    .param_types([u32::static_type(), u32::static_type()])
                    .build(),
            ]
        })
    }

    fn constructed(&self) {
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
