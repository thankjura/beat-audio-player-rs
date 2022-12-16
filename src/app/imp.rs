use std::cell::RefCell;
use std::sync::Arc;
use adw::subclass::prelude::AdwApplicationImpl;
use gettextrs::gettext;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::app::connector;
use crate::BeatWindow;
use crate::player::BeatPlayer;


pub struct BeatAppImp {
    pub window: RefCell<Option<BeatWindow>>,
    pub player: RefCell<Option<Arc<BeatPlayer>>>,
}

impl Default for BeatAppImp {
    fn default() -> Self {
        Self {
            window: RefCell::new(None),
            player: RefCell::new(None),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for BeatAppImp {
    const NAME: &'static str = "BeatApp";
    type Type = super::BeatApp;
    type ParentType = adw::Application;
}

impl ObjectImpl for BeatAppImp {}

impl ApplicationImpl for BeatAppImp {
    fn activate(&self) {
        self.parent_activate();

        let obj = self.obj();
        let window = BeatWindow::new(&*obj);
        window.set_title(Some(&gettext("Beat")));
        let player = Arc::new(BeatPlayer::default());
        obj.connect_shutdown(glib::clone!(@weak window, @weak player =>
            move |_| {
                player.destroy();
                window.destroy();
            }
        ));

        connector::connect(&window, &player);

        window.present();

        self.window.replace(Some(window));
        self.player.replace(Some(player.clone()));
    }
}

impl GtkApplicationImpl for BeatAppImp {}

impl AdwApplicationImpl for BeatAppImp {

}