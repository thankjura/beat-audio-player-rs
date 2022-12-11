use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use crate::player::BeatPlayer;
use crate::ui::{BeatNotebook, TrackRef};

pub struct QueueManager {
    player: BeatPlayer,
    notebook: RefCell<Option<Rc<BeatNotebook>>>
}

impl Default for QueueManager {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueManager {
    pub fn new() -> Self {
        Self {
            player: BeatPlayer::new(),
            notebook: RefCell::new(None)
        }
    }

    pub fn set_notebook(&self, notebook: Rc<BeatNotebook>) {
        self.notebook.borrow_mut().replace(notebook);
    }



    pub fn play(&self, tack_ref: TrackRef) {
        println!("{:#?}", tack_ref);

        let binding = self.notebook.borrow();
        let notebook = binding.borrow().as_ref().unwrap().as_ref();
        if let Some(track) = notebook.get_track(&tack_ref) {
            self.player.stop();
            self.player.set_uri(track.filepath());
            self.player.play();
            println!("{:#?}", track);
        }
    }
}