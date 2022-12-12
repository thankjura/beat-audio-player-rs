use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use crate::ui::window::notebook::BeatNotebook;

#[derive(Default, CompositeTemplate)]
#[template(file = "../../../resources/ui/window.ui")]
pub struct BeatWindowImp {
    #[template_child(id = "body")]
    pub body: TemplateChild<gtk::Box>,

    #[template_child(id = "footer")]
    pub footer: TemplateChild<gtk::Box>,

    #[template_child(id = "notebook")]
    pub notebook: TemplateChild<BeatNotebook>,

    #[template_child(id = "progress")]
    pub progress: TemplateChild<gtk::Scale>,

    #[template_child(id = "adjustment")]
    pub adjustment: TemplateChild<gtk::Adjustment>,

    #[template_child(id = "button_play_img")]
    pub button_play_img: TemplateChild<gtk::Image>,

    #[template_child(id = "current_position")]
    pub current_position_label: TemplateChild<gtk::Label>,

    #[template_child(id = "duration")]
    pub duration_label: TemplateChild<gtk::Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for BeatWindowImp {
    const NAME: &'static str = "BeatWindow";
    type Type = super::BeatWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BeatWindowImp {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("play").build(),
                Signal::builder("stop").build(),
                Signal::builder("next").build(),
                Signal::builder("prev").build(),
                Signal::builder("volume-changed").param_types([f64::static_type()]).build(),
                Signal::builder("open-path").param_types([Vec::<String>::static_type()]).build(),
            ]
        });

        SIGNALS.as_ref()
    }
}

impl WidgetImpl for BeatWindowImp {}

impl WindowImpl for BeatWindowImp {}

impl ApplicationWindowImpl for BeatWindowImp {}