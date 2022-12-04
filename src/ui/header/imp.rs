use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, FileChooserDialog, ResponseType};


#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/ui/header.ui")]
pub struct BeatHeader {

}


#[glib::object_subclass]
impl ObjectSubclass for BeatHeader {
    const NAME: &'static str = "BeatHeader";
    type Type = super::BeatHeader;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BeatHeader {
    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl BeatHeader {
    fn choose_files(&self, keep_tab: bool) {
        let dialog = FileChooserDialog::new(
            Some("Open folder"),
            gtk::Window::NONE,
            gtk::FileChooserAction::Open,
            &[("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );
        dialog.set_select_multiple(true);
        //dialog.set_modal(true);

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

impl WidgetImpl for BeatHeader {}

#[gtk::template_callbacks]
impl BeatHeader {
    #[template_callback]
    fn on_open_files(&self, _button: &gtk::Button) {
        println!("show dialog");
        self.choose_files(false);
    }

    #[template_callback]
    fn on_volume_changed(&self, value: f64) {

    }

    #[template_callback]
    fn on_add_files(&self, _button: &gtk::Button) {
        println!("show dialog");
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