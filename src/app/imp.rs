use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use gstreamer::prelude::ElementExt;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::BeatWindow;
use crate::player::BeatPlayer;


pub struct BeatAppImp {
    pub (super) window: RefCell<Option<Rc<BeatWindow>>>,
    pub player: Arc<BeatPlayer>,
}

impl Default for BeatAppImp {
    fn default() -> Self {
        Self {
            window: RefCell::new(None),
            player: Arc::new(BeatPlayer::new())
        }
    }
}


#[glib::object_subclass]
impl ObjectSubclass for BeatAppImp {
    const NAME: &'static str = "BeatApp";
    type Type = super::BeatApp;
    type ParentType = gtk::Application;
}

impl ObjectImpl for BeatAppImp {}

impl ApplicationImpl for BeatAppImp {
    fn activate(&self) {
        self.parent_activate();

        let obj = self.obj();
        let window = BeatWindow::new(&*obj);
        window.set_title(Some("Beat"));

        let window = Rc::new(window);
        let player = self.player.clone();

        obj.connect_shutdown(glib::clone!(@weak window, @weak player =>
            move |_| {
                player.destroy();
                window.destroy();
            }
        ));

        self.window.replace(Some(window.clone()));

        let bus = player.pipeline.bus().unwrap();
        bus.add_signal_watch();

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        self.watch_bus(sender.clone(), &bus, &player);
        self.link_header(&window, &player);
        self.link_notebook(sender.clone(), &window, &player);
        let progress_signal = self.link_progress(sender.clone(), &window, &player);
        self.watch_channel(receiver, &window, progress_signal);
        window.present();
    }
}

impl GtkApplicationImpl for BeatAppImp {}