use std::cell::RefCell;
use std::rc::Rc;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::BeatWindow;


pub struct BeatAppImp {
    pub (super) window: RefCell<Option<Rc<BeatWindow>>>
}

impl Default for BeatAppImp {
    fn default() -> Self {
        Self {
            window: RefCell::new(None)
        }
    }
}


#[glib::object_subclass]
impl ObjectSubclass for BeatAppImp {
    const NAME: &'static str = "BeatApp";
    type Type = super::BeatApp;
    type ParentType = gtk::Application;
}

impl ApplicationImpl for BeatAppImp {
    fn activate(&self) {
        let obj = self.obj();
        let window = BeatWindow::new(&*obj);
        window.set_title(Some("Beat"));
        window.setup_actions();
        let window = Rc::new(window);



        obj.connect_shutdown(glib::clone!(@weak window =>
            move |_| {
                window.destroy();
            }
        ));
        self.window.replace(Some(window.clone()));
        window.present();
    }
}

impl ObjectImpl for BeatAppImp {}

impl GtkApplicationImpl for BeatAppImp {}