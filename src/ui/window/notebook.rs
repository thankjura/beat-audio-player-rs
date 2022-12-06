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

    pub fn add_tab(&self, name: &str) {
        let tab = Tab::new(name);
        let idx = self.notebook.get().append_page(tab.playlist().scrollbox(), Some(tab.label()));
        self.notebook.get().set_show_tabs(true);
        self.notebook.get().set_current_page(Some(idx));
        self.tabs.borrow_mut().push(tab);
    }

    pub fn current_playlist(&self) {
        if let Some(tab_index) = self.notebook.current_page() {

        }
    }
}