//! StatusNotifierItem tray icon (KDE, Waybar, XFCE, Cinnamon, GNOME with the
//! AppIndicator extension).
//!
//! ksni runs the D-Bus service on its own executor thread, so every user action
//! is forwarded to the GTK main thread as a [`Command`].

use super::Command;
use crate::APP_ID;
use gettextrs::gettext;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib;
use ksni::menu::{CheckmarkItem, StandardItem};
use ksni::{Category, Icon, MenuItem, Orientation, ToolTip, TrayMethods};

const ICON_RESOURCE: &str = "/ru/slie/beat/icons/beat.svg";
const ICON_SIZES: [i32; 6] = [16, 22, 24, 32, 48, 64];

pub struct BeatTray {
    commands: async_channel::Sender<Command>,
    icons: Vec<Icon>,
    playing: bool,
    now_playing: Option<String>,
    window_visible: bool,
    close_to_tray: bool,
}

impl BeatTray {
    fn send(&self, command: Command) {
        let _ = self.commands.try_send(command);
    }

    fn command_item(label: String, icon_name: &str, command: Command) -> MenuItem<Self> {
        StandardItem {
            label,
            icon_name: icon_name.into(),
            activate: Box::new(move |this: &mut Self| this.send(command.clone())),
            ..Default::default()
        }
        .into()
    }
}

/// Renders the application icon into ARGB32 (network byte order) pixmaps. The
/// icon is shipped as a pixmap rather than a theme name so the tray shows it
/// even when the app is not installed system-wide.
fn icon_pixmaps() -> Vec<Icon> {
    ICON_SIZES
        .iter()
        .filter_map(|&size| Pixbuf::from_resource_at_scale(ICON_RESOURCE, size, size, true).ok())
        .map(|pixbuf| {
            let pixbuf = pixbuf.add_alpha(false, 0, 0, 0).unwrap_or(pixbuf);
            let (width, height) = (pixbuf.width(), pixbuf.height());
            let stride = pixbuf.rowstride() as usize;
            let bytes = pixbuf.read_pixel_bytes();
            let mut data = Vec::with_capacity((width * height * 4) as usize);
            for row in bytes.chunks(stride).take(height as usize) {
                for rgba in row[..width as usize * 4].as_chunks::<4>().0 {
                    data.extend_from_slice(&[rgba[3], rgba[0], rgba[1], rgba[2]]);
                }
            }
            Icon {
                width,
                height,
                data,
            }
        })
        .collect()
}

impl ksni::Tray for BeatTray {
    fn id(&self) -> String {
        APP_ID.into()
    }

    fn title(&self) -> String {
        "Beat".into()
    }

    fn category(&self) -> Category {
        Category::ApplicationStatus
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        self.icons.clone()
    }

    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            title: "Beat".into(),
            description: self.now_playing.clone().unwrap_or_default(),
            ..Default::default()
        }
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        self.send(Command::ToggleWindow);
    }

    fn secondary_activate(&mut self, _x: i32, _y: i32) {
        self.send(Command::PlayPause);
    }

    fn scroll(&mut self, delta: i32, orientation: Orientation) {
        if orientation == Orientation::Vertical && delta != 0 {
            // Hosts disagree on the sign and size of delta; only the direction matters.
            self.send(Command::StepVolume(-delta.signum()));
        }
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let (play_label, play_icon) = if self.playing {
            (gettext("Pause"), "media-playback-pause-symbolic")
        } else {
            (gettext("Play"), "media-playback-start-symbolic")
        };
        let window_label = if self.window_visible {
            gettext("Hide window")
        } else {
            gettext("Show window")
        };

        let mut items = vec![];
        if let Some(now_playing) = &self.now_playing {
            items.push(
                StandardItem {
                    label: now_playing.replace('_', "__"),
                    enabled: false,
                    ..Default::default()
                }
                .into(),
            );
            items.push(MenuItem::Separator);
        }
        items.extend([
            Self::command_item(play_label, play_icon, Command::PlayPause),
            Self::command_item(
                gettext("Stop"),
                "media-playback-stop-symbolic",
                Command::Stop,
            ),
            Self::command_item(
                gettext("Previous"),
                "media-skip-backward-symbolic",
                Command::Prev,
            ),
            Self::command_item(
                gettext("Next"),
                "media-skip-forward-symbolic",
                Command::Next,
            ),
            MenuItem::Separator,
            StandardItem {
                label: window_label,
                activate: Box::new(|this: &mut Self| {
                    let command = if this.window_visible {
                        Command::ToggleWindow
                    } else {
                        Command::Raise
                    };
                    this.send(command);
                }),
                ..Default::default()
            }
            .into(),
            CheckmarkItem {
                label: gettext("Close to tray"),
                checked: self.close_to_tray,
                activate: Box::new(|this: &mut Self| {
                    this.close_to_tray = !this.close_to_tray;
                    this.send(Command::SetCloseToTray(this.close_to_tray));
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            Self::command_item(gettext("Quit"), "application-exit-symbolic", Command::Quit),
        ]);
        items
    }

    fn watcher_online(&self) {
        self.send(Command::TrayAvailable(true));
    }

    fn watcher_offline(&self, _reason: ksni::OfflineReason) -> bool {
        self.send(Command::TrayAvailable(false));
        // Keep the service alive so the icon comes back with the panel.
        true
    }
}

type TrayUpdate = Box<dyn FnOnce(&mut BeatTray) + Send>;

pub struct Tray {
    updates: async_channel::Sender<TrayUpdate>,
}

/// Flatpak only lets the app own names under its own id, so the
/// `org.kde.StatusNotifierItem-PID-ID` name can't be taken there.
fn is_sandboxed() -> bool {
    std::path::Path::new("/.flatpak-info").exists()
}

impl Tray {
    pub async fn spawn(
        commands: async_channel::Sender<Command>,
        close_to_tray: bool,
        window_visible: bool,
    ) -> Result<Self, ksni::Error> {
        let tray = BeatTray {
            commands,
            icons: icon_pixmaps(),
            playing: false,
            now_playing: None,
            window_visible,
            close_to_tray,
        };
        let handle = tray.disable_dbus_name(is_sandboxed()).spawn().await?;

        // Apply updates one at a time so they reach the tray in order.
        let (updates, receiver) = async_channel::unbounded::<TrayUpdate>();
        glib::spawn_future_local(async move {
            while let Ok(update) = receiver.recv().await {
                handle.update(update).await;
            }
        });

        Ok(Self { updates })
    }

    fn update(&self, f: impl FnOnce(&mut BeatTray) + Send + 'static) {
        let _ = self.updates.try_send(Box::new(f));
    }

    pub fn set_playback(&self, playing: bool, now_playing: Option<String>) {
        self.update(move |tray| {
            tray.playing = playing;
            tray.now_playing = now_playing;
        });
    }

    pub fn set_window_visible(&self, visible: bool) {
        self.update(move |tray| tray.window_visible = visible);
    }
}
