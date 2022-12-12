use std::rc::Rc;
use std::sync::Arc;
use gtk::glib::Sender;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::app::imp::BeatAppImp;
use crate::app::relation::channel::AppMessage;
use crate::BeatWindow;
use crate::player::BeatPlayer;
use crate::structs::track::TrackState;
use crate::ui::BeatNotebook;

impl BeatAppImp {
    pub fn link_notebook(&self, sender: Sender<AppMessage>, window: &Rc<BeatWindow>, player: &Arc<BeatPlayer>) {
        let player_weak = Arc::downgrade(player);
        window.imp().notebook.get().connect("track_activated", false, move |values| {
            let notebook = values[0].get::<&BeatNotebook>().unwrap();
            let tab_idx = values[1].get::<u32>().unwrap();
            let track_idx = values[2].get::<u32>().unwrap();
            if let Some(track) = notebook.get_track(tab_idx, track_idx) {
                let player = player_weak.upgrade().unwrap();
                player.set_uri(track.filepath());
                player.play();

                sender.send(AppMessage::TrackChanged(tab_idx, track_idx, TrackState::Playing));
            }
            None
        });
    }
}