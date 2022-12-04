use std::cell::Cell;
use std::fs::File;
use std::rc::Rc;
use crate::app::player::BeatPlayer;
use crate::app::playlist::{PlayList, PlayListItem, PlayListPosition};

pub struct BeatApp {
    player: BeatPlayer,
    playlists: Vec<PlayList>,
    current_playlist: usize,
}

impl BeatApp {
    pub fn new() -> Self {
        let playlist = PlayList::new("new");
        let playlists = vec![playlist];
        let player = BeatPlayer::build();

        BeatApp {
            player,
            playlists,
            current_playlist: 0,
        }
    }

    fn new_playlist(&mut self, name: &str) -> &mut PlayList {
        let ps = PlayList::new(name);
        self.playlists.push(ps);
        self.current_playlist = self.playlists.len() - 1;
        self.playlists.get_mut(self.current_playlist).unwrap()
    }

    fn current_playlist(&self) -> &PlayList {
        if self.playlists.len() > self.current_playlist {
            self.playlists.get(self.current_playlist).unwrap()
        } else {
            self.playlists.get(0).unwrap()
        }
    }

    fn current_playlist_mut(&mut self) -> &mut PlayList {
        if self.playlists.len() > self.current_playlist {
            self.playlists.get_mut(self.current_playlist).unwrap()
        } else {
            self.playlists.get_mut(0).unwrap()
        }
    }

    fn drop_playlist(&mut self, index: usize) {
        self.playlists.remove(index);
    }

    pub fn set_current_playlist(&mut self, index: usize) -> Option<&mut PlayList> {
        if self.playlists.len() > index {
            self.current_playlist = index;
        } else {
            self.current_playlist = 0;
        }

        self.playlists.get_mut(self.current_playlist)
    }

    pub fn set_position(&mut self, pos: &PlayListPosition) {
        //let player = Cell::new(&self.player);

        if let Some(playlist) = self.set_current_playlist(pos.playlist_idx()) {
            if let Some(item) = playlist.set_item(pos.item_idx()) {
                //player.get().set_uri(item.path());
            }
        }
    }

    pub fn open_one(&mut self, path: &str, keep_tab: bool) -> Option<PlayListPosition> {
        let playlist: &mut PlayList;

        if keep_tab {
            playlist = self.current_playlist_mut();
        } else {
            playlist = self.new_playlist("new");
        }

        if let Ok(file) = File::open(path) {
            let item_index = playlist.add_file(path);
            return Some(PlayListPosition::new(self.current_playlist, item_index));
        }

        None
    }

    pub fn get_current_item(&self) -> Option<&PlayListItem> {
        self.current_playlist().get_current()
    }

    pub fn open_many(&self, paths: &[&str], keep_tab: bool) {

    }

    pub fn play(&mut self) {
        let player = Rc::new(Cell::new(&self.player));

        if let Some(item) = self.get_current_item() {
            player.get().set_uri(item.path());
            player.get().play();
        }
    }

    fn pause(&self) {
        self.player.pause();
    }

    fn next(&self) {

    }

    fn prev(&self) {

    }

    pub fn destroy(&self) {
        self.player.destroy();
    }
}