use crate::app::connector;
use crate::app::remote::Remote;
use crate::player::BeatPlayer;
use crate::utils::settings::BeatSettings;
use crate::BeatWindow;
use adw::subclass::prelude::AdwApplicationImpl;
use gettextrs::gettext;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct BeatAppImp {
    pub window: RefCell<Option<Rc<BeatWindow>>>,
    pub player: RefCell<Option<Arc<BeatPlayer>>>,
    pub settings: RefCell<Option<Arc<Mutex<BeatSettings>>>>,
    pub remote: RefCell<Option<Rc<Remote>>>,
}

impl Default for BeatAppImp {
    fn default() -> Self {
        Self {
            window: RefCell::new(None),
            player: RefCell::new(None),
            settings: RefCell::new(None),
            remote: RefCell::new(None),
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
    fn startup(&self) {
        self.parent_startup();

        // With "close to tray" the window close button no longer quits.
        let obj = self.obj();
        let quit = gio::ActionEntry::builder("quit")
            .activate(|app: &super::BeatApp, _, _| app.quit())
            .build();
        obj.add_action_entries([quit]);
        obj.set_accels_for_action("app.quit", &["<Control>q"]);
    }

    fn activate(&self) {
        self.parent_activate();

        let display = gtk::gdk::Display::default().expect("GTK display must be available");
        gtk::IconTheme::for_display(&display).add_resource_path("/ru/slie/beat/icons");

        let obj = self.obj();
        let window = BeatWindow::new(&*obj);
        window.set_title(Some(&gettext("Beat")));
        let player = Arc::new(BeatPlayer::default());
        obj.connect_shutdown(glib::clone!(
            #[weak]
            window,
            #[weak]
            player,
            move |_| {
                player.destroy();
                window.destroy();
            }
        ));

        let mut settings = BeatSettings::load();

        for playlist in settings.playlists() {
            let tab = window
                .imp()
                .notebook
                .get()
                .imp()
                .add_tab_with_uuid(&playlist.label, &playlist.uuid);
            for track in playlist.rows {
                tab.add_track(track);
            }
        }

        let settings = Arc::new(Mutex::new(settings));

        let remote = Remote::new(&obj);
        connector::connect(&window, &player, &settings, &remote);

        let window = Rc::new(window);

        self.window.replace(Some(window.clone()));
        self.player.replace(Some(player));
        self.settings.replace(Some(settings));

        window.present();

        // Needs the window and settings registered above.
        remote.start();
        self.remote.replace(Some(remote));
    }
}

impl GtkApplicationImpl for BeatAppImp {}

impl AdwApplicationImpl for BeatAppImp {}
