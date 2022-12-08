use std::rc::Rc;
use crate::ui::window::notebook::tab::Tab;
use crate::ui::window::notebook::imp::BeatNotebookImp;

impl BeatNotebookImp {
    pub fn toggle_show_tabs(&self) {
        if self.notebook.n_pages() > 1 {
            self.notebook.set_show_tabs(true);
        } else {
            self.notebook.set_show_tabs(false);
        }
    }

    pub fn add_tab(&self, name: &str) -> Rc<Tab> {
        let tab = Rc::new(Tab::new(name));
        let idx = self.notebook.append_page(tab.scrollbox(), Some(tab.label()));
        self.notebook.set_show_tabs(true);
        self.notebook.set_current_page(Some(idx));
        self.tabs.borrow_mut().push(tab.clone());

        tab
    }

    pub fn selected_tab(&self) -> Rc<Tab> {
        if let Some(tab_index) = self.notebook.current_page() {
            return self.tabs.borrow().get(usize::try_from(tab_index).unwrap()).unwrap().clone();
        };

        self.add_tab("new")
    }
}