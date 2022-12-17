use std::rc::Rc;
use gtk::{gio, glib};
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use uuid::Uuid;
use crate::ui::BeatNotebook;
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

    pub fn add_tab_wth_uuid(&self, name: &str, uuid: &str) -> Rc<Tab> {
        let tab = Rc::new(Tab::new(name, uuid));

        // Actions
        let action_group = gio::SimpleActionGroup::new();
        let close_action = gio::SimpleAction::new("close", None);
        action_group.add_action(&close_action);
        tab.widget().insert_action_group("tab", Some(&action_group));

        close_action.connect_activate(glib::clone!(@weak tab => move |_action, _value| {
            if let Some(notebook) = tab.widget().ancestor(BeatNotebook::static_type()) {
                let notebook = notebook.downcast::<BeatNotebook>();
                if let Ok(notebook) = &notebook {
                    let tabs_count = notebook.imp().tabs.borrow().len();

                    let value = notebook.imp().tabs.borrow().iter().position(|item| Rc::ptr_eq(item, &tab));
                    if let Some(value) = value {
                        let tab_idx = value as u32;
                        notebook.emit_by_name::<()>("tab-removed", &[&tab_idx]);
                        if tabs_count > 1 {
                            notebook.imp().notebook.remove_page(Some(tab_idx));
                            notebook.imp().tabs.borrow_mut().remove(value);
                        } else {
                            tab.clear_tab();
                        }
                    }
                }
            }
        }));
        // End actions

        let idx = self.notebook.append_page(tab.scrollbox(), Some(tab.widget()));

        self.toggle_show_tabs();
        self.notebook.set_current_page(Some(idx));
        self.tabs.borrow_mut().push(tab.clone());

        tab
    }

    pub fn add_tab(&self, name: &str) -> Rc<Tab> {
        let uuid = Uuid::new_v4().to_string();
        let tab = Rc::new(Tab::new(name, &uuid));

        // Actions
        let action_group = gio::SimpleActionGroup::new();
        let close_action = gio::SimpleAction::new("close", None);
        action_group.add_action(&close_action);
        tab.widget().insert_action_group("tab", Some(&action_group));

        close_action.connect_activate(glib::clone!(@weak tab => move |_action, _value| {
            if let Some(notebook) = tab.widget().ancestor(BeatNotebook::static_type()) {
                let notebook = notebook.downcast::<BeatNotebook>();
                if let Ok(notebook) = &notebook {
                    let tabs_count = notebook.imp().tabs.borrow().len();

                    let value = notebook.imp().tabs.borrow().iter().position(|item| Rc::ptr_eq(item, &tab));
                    if let Some(value) = value {
                        let tab_idx = value as u32;
                        notebook.emit_by_name::<()>("tab-removed", &[&tab_idx]);
                        if tabs_count > 1 {
                            notebook.imp().notebook.remove_page(Some(tab_idx));
                            notebook.imp().tabs.borrow_mut().remove(value);
                        } else {
                            tab.clear_tab();
                        }
                    }
                }
            }
        }));
        // End actions

        let idx = self.notebook.append_page(tab.scrollbox(), Some(tab.widget()));

        self.toggle_show_tabs();
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

    pub fn get_tab(&self, idx: usize) -> Option<Rc<Tab>> {
        if let Some(tab) = self.tabs.borrow().get(idx) {
            return Some(tab.clone());
        }

        None
    }

    pub fn selected_tab_id(&self) -> Option<u32> {
        self.notebook.current_page()
    }

    pub fn active_tab_track(&self) -> Option<(u32, Rc<Tab>, u32)> {
        for (idx, tab) in self.tabs.borrow().iter().enumerate() {
            if let Some(index) = tab.active_track() {
                return Some((idx as u32, tab.clone(), index));
            }
        }

        None
    }
}