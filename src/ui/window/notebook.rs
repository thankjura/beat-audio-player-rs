use crate::ui::playlist::PlayList;
use crate::ui::window::tab::Tab;
use crate::ui::window::widget::BeatWindow;


impl BeatWindow {
    pub fn toggle_show_tabs(&self) {
        if self.notebook.get().n_pages() > 1 {
            self.notebook.get().set_show_tabs(true);
        } else {
            self.notebook.get().set_show_tabs(false);
        }
    }

    pub fn add_tab(&self) {
        let playlist = PlayList::new_with_uuid(&uuid::Uuid::new_v4().to_string());
        let tab = Tab::new_with_label("new");
        let idx = self.notebook.get().append_page(playlist.scrollbox(), Some(tab.label()));
        self.notebook.get().set_show_tabs(true);
        self.notebook.get().set_current_page(Some(idx));
        println!("Tab added");
        //self.tabs.push(tab);
    }
}