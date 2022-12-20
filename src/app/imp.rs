use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use adw::subclass::prelude::AdwApplicationImpl;
use gettextrs::gettext;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::app::connector;
use crate::BeatWindow;
use crate::player::BeatPlayer;
use crate::utils::settings::BeatSettings;


pub struct BeatAppImp {
    pub window: RefCell<Option<Rc<BeatWindow>>>,
    pub player: RefCell<Option<Arc<BeatPlayer>>>,
    pub settings: RefCell<Option<Arc<Mutex<BeatSettings>>>>,
}

impl Default for BeatAppImp {
    fn default() -> Self {
        Self {
            window: RefCell::new(None),
            player: RefCell::new(None),
            settings: RefCell::new(None),
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

        let mut settings = BeatSettings::load();

        for playlist in settings.playlists() {
            let tab = window.imp().notebook.get().imp().add_tab_wth_uuid(&playlist.label, &playlist.uuid);
            for track in playlist.rows {
                tab.add_track(track);
            }
        }

        let settings = Arc::new(Mutex::new(settings));

        connector::connect(&window, &player, &settings);

        let window = Rc::new(window);

        self.window.replace(Some(window.clone()));
        self.player.replace(Some(player.clone()));
        self.settings.replace(Some(settings));

        window.present();
    }
}

impl GtkApplicationImpl for BeatAppImp {}

impl AdwApplicationImpl for BeatAppImp {

}