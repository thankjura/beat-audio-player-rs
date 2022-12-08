use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{FileChooserDialog, ResponseType};
use crate::ui::window::imp::BeatWindowImp;

impl BeatWindowImp {
    fn choose_files(&self, _keep_tab: bool) {
        let binding = self.instance();
        let w = binding.as_ref();
        let dialog = FileChooserDialog::new(
            Some("Open folder"),
            Some(w),
            gtk::FileChooserAction::Open,
            &[("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );

        dialog.set_select_multiple(true);
        dialog.set_modal(true);

        let audio_filter = gtk::FileFilter::new();
        audio_filter.add_mime_type("audio/*");
        audio_filter.add_mime_type("inode/directory");
        audio_filter.set_name(Some("Audio files or directory"));
        dialog.add_filter(&audio_filter);

        dialog.connect_response(move |d: &FileChooserDialog, response: ResponseType| {
            if response == ResponseType::Ok {
                let file = d.file().unwrap();
                let path = file.path().unwrap();
                println!("{:#?}", path);
            }

            d.close();
        });

        dialog.show();
    }
}

#[gtk::template_callbacks]
impl BeatWindowImp {
    #[template_callback]
    fn on_open_files(&self, _button: &gtk::Button) {

        self.choose_files(false);
    }

    #[template_callback]
    fn on_volume_changed(&self, _value: f64) {

    }

    #[template_callback]
    fn on_add_files(&self, _button: &gtk::Button) {
        self.choose_files(true);
    }

    #[template_callback]
    fn on_stop(&self, _button: &gtk::Button) {

    }

    #[template_callback]
    fn on_play(&self, _button: &gtk::Button) {

    }

    #[template_callback]
    fn on_prev(&self, _button: &gtk::Button) {

    }

    #[template_callback]
    fn on_next(&self, _button: &gtk::Button) {

    }
}