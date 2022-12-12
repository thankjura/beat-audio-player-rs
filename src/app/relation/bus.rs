use std::sync::Arc;
use gstreamer::{Bus, MessageView, State};
use gtk::glib::Sender;
use crate::app::imp::BeatAppImp;
use crate::app::relation::channel::AppMessage;
use crate::player::BeatPlayer;

impl BeatAppImp {
    pub fn watch_bus(&self, sender: Sender<AppMessage>, bus: &Bus, player: &Arc<BeatPlayer>) {
        let player_weak = Arc::downgrade(player);

        let sender_ref = sender.clone();

        bus.connect_message(Some("state-changed"), move |_bus, msg| {
            if let MessageView::StateChanged(value) = msg.view() {
                if let Some(src) = msg.src() {
                    if src != player_weak.upgrade().unwrap().pipeline {
                        return;
                    }
                }
                let value = value.current() == State::Playing;
                if let Err(_) = sender_ref.send(AppMessage::PlayingState(value)) {
                    println!("error send icon change message");
                }
            };
        });

        let player_weak = Arc::downgrade(player);

        bus.connect_message(Some("stream-start"), move |_bus, msg| {
            if let MessageView::StreamStart(value) = msg.view() {
                if let Some(src) = msg.src() {
                    let player = player_weak.upgrade().unwrap();
                    if src == player.pipeline {
                        println!("Start stream");
                        if let Some(duration) = player.get_duration() {
                            sender.send(AppMessage::DurationChanged(duration));
                        }
                    }
                }
            }
        });
    }
}