use std::rc::Rc;
use std::sync::Arc;
use gtk::glib;
use gtk::glib::{Sender, SignalHandlerId};
use gtk::prelude::RangeExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::app::imp::BeatAppImp;
use crate::app::relation::channel::AppMessage;
use crate::BeatWindow;
use crate::player::BeatPlayer;

impl BeatAppImp {
    pub fn link_progress(&self, sender: Sender<AppMessage>, window: &Rc<BeatWindow>, player: &Arc<BeatPlayer>) -> SignalHandlerId {
        let player_weak = Arc::downgrade(player);
        let progress_value_signal = window.imp().progress.get().connect_value_changed(move |scale| {
            let player = player_weak.upgrade().unwrap();
            player.set_position_percent(scale.value());
        });

        let player_weak = Arc::downgrade(player);

        glib::timeout_add_seconds(1, move || {
            let player = player_weak.upgrade().unwrap();
            if let Some((position, progress)) = player.get_position_progress() {
                //sender.send(AppMessage::DurationChanged(duration));
                sender.send(AppMessage::ProgressChanged(position, progress));
            } else {
                sender.send(AppMessage::ClearProgress);
            }

            glib::Continue(true)
        });

        progress_value_signal
    }
}