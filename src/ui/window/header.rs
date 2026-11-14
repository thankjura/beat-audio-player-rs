use crate::structs::action::Action;
use crate::ui::window::imp::BeatWindowImp;
use gettextrs::gettext;
use gtk::gio;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

impl BeatWindowImp {
    fn choose_files(&self, keep_tab: bool) {
        let window = self.obj();
        let dialog = gtk::FileDialog::builder()
            .title(gettext("Choose audio files or folder"))
            .accept_label(gettext("Open"))
            .modal(true)
            .build();

        let audio_filter = gtk::FileFilter::new();
        audio_filter.add_mime_type("audio/*");
        audio_filter.add_mime_type("inode/directory");
        audio_filter.set_name(Some("Audio files or directory"));

        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&audio_filter);
        dialog.set_filters(Some(&filters));
        dialog.set_default_filter(Some(&audio_filter));

        let window_ref = self.downgrade();
        dialog.open_multiple(
            Some(window.as_ref()),
            None::<&gio::Cancellable>,
            move |result| {
                let Ok(files) = result else {
                    return;
                };
                let paths = files
                    .iter::<gio::File>()
                    .filter_map(Result::ok)
                    .filter_map(|file| file.path())
                    .filter_map(|path| path.into_os_string().into_string().ok())
                    .collect();

                if let Some(window) = window_ref.upgrade() {
                    window.open_path(paths, keep_tab);
                }
            },
        );
    }

    pub fn set_playing_icon(&self, value: bool) {
        if value {
            self.button_play_img
                .get()
                .set_icon_name(Some("media-playback-pause-symbolic"))
        } else {
            self.button_play_img
                .get()
                .set_icon_name(Some("media-playback-start-symbolic"));
        }
    }
}

#[gtk::template_callbacks]
impl BeatWindowImp {
    #[template_callback]
    fn on_open_files(&self, _button: &gtk::Button) {
        self.choose_files(false);
    }

    #[template_callback]
    fn on_volume_changed(&self, value: f64) {
        self.obj().emit_by_name::<()>("volume-changed", &[&value]);
    }

    #[template_callback]
    fn on_add_files(&self, _button: &gtk::Button) {
        self.choose_files(true);
    }

    #[template_callback]
    fn on_stop(&self, _button: &gtk::Button) {
        self.obj()
            .emit_by_name::<()>("action", &[&Action::Stop.get_value()]);
    }

    #[template_callback]
    fn on_play(&self, _button: &gtk::Button) {
        self.obj()
            .emit_by_name::<()>("action", &[&Action::Play.get_value()]);
    }

    #[template_callback]
    fn on_prev(&self, _button: &gtk::Button) {
        self.obj()
            .emit_by_name::<()>("action", &[&Action::Prev.get_value()]);
    }

    #[template_callback]
    fn on_next(&self, _button: &gtk::Button) {
        self.obj()
            .emit_by_name::<()>("action", &[&Action::Next.get_value()]);
    }
}
