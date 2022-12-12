use std::rc::Rc;
use std::sync::Arc;
use gtk::prelude::*;
use crate::app::imp::BeatAppImp;
use crate::BeatWindow;
use crate::player::BeatPlayer;

impl BeatAppImp {
    pub fn link_header(&self, window: &Rc<BeatWindow>, player: &Arc<BeatPlayer>) {
        // Connect to stop button
        let player_weak = Arc::downgrade(player);
        window.connect("stop", false, move |_value| {
            let player = player_weak.upgrade().unwrap();
            player.stop();
            None
        });

        // Connect to play button
        let player_weak = Arc::downgrade(player);
        window.connect("play", false, move |_value| {
            let player = player_weak.upgrade().unwrap();
            player.toggle_play();
            None
        });


        // Connect to volume
        let player_weak = Arc::downgrade(player);

        window.connect("volume-changed", false, move |values| {
            let player = player_weak.upgrade().unwrap();
            let value = values[1].get::<f64>().unwrap();
            player.set_volume(value);
            None
        });
    }
}