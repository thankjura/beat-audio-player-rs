use crate::structs::action::Action;
use crate::ui::window::imp::BeatWindowImp;
use gettextrs::gettext;
use gtk::gio;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

impl BeatWindowImp {
    fn choose_files(&self, _keep_tab: bool) {
        let binding = self.instance();
        let w = binding.as_ref();
        let dialog = gtk::FileChooserDialog::new(
            Some(&gettext("Choose audio files or folder")),
            Some(w),
            gtk::FileChooserAction::Open,
            &[
                (&gettext("Open"), gtk::ResponseType::Ok),
                (&gettext("Cancel"), gtk::ResponseType::Cancel),
            ],
        );

        dialog.set_select_multiple(true);
        dialog.set_modal(true);

        let audio_filter = gtk::FileFilter::new();
        audio_filter.add_mime_type("audio/*");
        audio_filter.add_mime_type("inode/directory");
        audio_filter.set_name(Some("Audio files or directory"));
        dialog.add_filter(&audio_filter);

        let window_ref = self.downgrade();
        dialog.connect_response(move |d, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                let mut files = vec![];

                for file in d.files().iter::<gio::File>().unwrap() {
                    if let Ok(file) = file {
                        files.push(file.path().unwrap().to_str().unwrap().to_string());
                    }
                }
                window_ref.upgrade().unwrap().open_path(files, _keep_tab);
            }

            d.close();
        });

        dialog.show();
    }

    pub fn set_playing_icon(&self, value: bool) {
        if value {
            self.button_play_img
                .get()
                .set_from_icon_name(Some("media-playback-pause-symbolic"))
        } else {
            self.button_play_img
                .get()
                .set_from_icon_name(Some("media-playback-start-symbolic"));
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
        self.instance()
            .emit_by_name::<()>("volume-changed", &[&value]);
    }

    #[template_callback]
    fn on_add_files(&self, _button: &gtk::Button) {
        self.choose_files(true);
    }

    #[template_callback]
    fn on_stop(&self, _button: &gtk::Button) {
        self.instance()
            .emit_by_name::<()>("action", &[&Action::STOP.get_value()]);
    }

    #[template_callback]
    fn on_play(&self, _button: &gtk::Button) {
        self.instance()
            .emit_by_name::<()>("action", &[&Action::PLAY.get_value()]);
    }

    #[template_callback]
    fn on_prev(&self, _button: &gtk::Button) {
        self.instance()
            .emit_by_name::<()>("action", &[&Action::PREV.get_value()]);
    }

    #[template_callback]
    fn on_next(&self, _button: &gtk::Button) {
        self.instance()
            .emit_by_name::<()>("action", &[&Action::NEXT.get_value()]);
    }
}
