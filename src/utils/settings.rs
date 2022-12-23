use configparser::ini::Ini;
use gtk::glib;
use lazy_static::lazy_static;
use regex::Regex;
use serde;
use std::fs::File;
use std::{fs, path::PathBuf};

use crate::structs::track::Track;

pub struct BeatSettings {
    config: Ini,
    config_dir: PathBuf,
    path: PathBuf,
}

lazy_static! {
    static ref REGEXP: Regex =
        Regex::new(r"^[a-f0-9]{8}-?[a-f0-9]{4}-?4[a-f0-9]{3}-?[89ab][a-f0-9]{3}-?[a-f0-9]{12}")
            .unwrap();
}

#[derive(Debug)]
pub struct TabData {
    pub rows: Vec<Track>,
    pub uuid: String,
    pub label: String,
    position: u32,
    pub selected: bool,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
struct PlRow {
    src: String,
    artist: Option<String>,
    album: Option<String>,
    title: Option<String>,
    length: Option<String>,
}

impl BeatSettings {
    pub fn load() -> Self {
        let config_dir = glib::user_config_dir().join("beat");
        fs::create_dir_all(&config_dir).expect("Can't create config directory");
        let config_path = config_dir.join("config.ini");

        let mut config = Ini::new();
        if config_path.is_file() {
            config.load(&config_path).expect("can't parse local config");
        }
        Self {
            config,
            config_dir,
            path: config_path,
        }
    }

    pub fn save(&self) {
        self.config
            .write(&self.path)
            .expect("Can't save configuration");
    }

    pub fn drop_playlist(&mut self, uuid: &str) {
        if let Some(section) = self.config.remove_section(uuid) {
            if let Some(Some(filename)) = section.get("file") {
                let path = PathBuf::from(filename);
                if path.is_file() {
                    if let Err(_) = fs::remove_file(path) {
                        println!("Can't remove playlist file");
                    }
                }
            }
        }

        self.save();
    }

    pub fn save_playlist(&mut self, uuid: &str, name: &str, tracks: Vec<Track>) {
        self.config.set(uuid, "label", Some(name.to_string()));
        let mut filename = self.config.get(uuid, "file");
        if filename.is_none() {
            filename.replace(uuid.to_string() + ".csv");
            self.config.set(uuid, "file", filename.clone());
        }
        self.save();

        let mut filename = PathBuf::from(filename.unwrap());
        if !filename.is_absolute() {
            filename = self.config_dir.join(filename);
        }

        if let Ok(file) = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filename)
        {
            let mut writer = csv::Writer::from_writer(file);
            for track in tracks {
                writer
                    .serialize(PlRow {
                        src: track.filepath().to_string(),
                        artist: track.get_by_name("artist").map(|s| s.to_string()),
                        album: track.get_by_name("album").map(|s| s.to_string()),
                        title: track.get_by_name("title").map(|s| s.to_string()),
                        length: track.get_by_name("length").map(|s| s.to_string()),
                    })
                    .unwrap();
            }
        } else {
            println!("Can't save playlist");
        }
    }

    pub fn playlists(&mut self) -> Vec<TabData> {
        let mut out = vec![];
        let mut need_save = false;

        let selected_tab = self
            .config
            .get("main", "selected")
            .unwrap_or("000".to_string());

        for section in self.config.sections().iter() {
            if REGEXP.is_match(section) {
                let section_map = self.config.get_map_ref()[section].clone();

                if let Some(Some(playlist_path)) = &section_map.get("file").clone() {
                    let playlist_path = PathBuf::from(self.config_dir.join(playlist_path));
                    if let Ok(_file) = File::open(&playlist_path) {
                        let mut tracks = vec![];
                        let position;
                        let label;
                        if let Some(Some(pos)) = section_map.get("position").clone() {
                            if let Ok(pos) = pos.parse::<u32>() {
                                position = pos;
                            } else {
                                position = 0;
                            }
                        } else {
                            position = 0;
                        }

                        if let Some(Some(l)) = section_map.get("label").clone() {
                            label = l.to_string();
                        } else {
                            label = "unknown".to_string();
                        }

                        if let Ok(mut reader) = csv::Reader::from_path(&playlist_path) {
                            for r in reader.deserialize::<PlRow>() {
                                if let Ok(r) = r {
                                    let r: PlRow = r;
                                    let path = PathBuf::from(r.src);
                                    tracks.push(Track::new(
                                        path.file_name().unwrap().to_str().unwrap(),
                                        path.to_str().unwrap(),
                                        r.album.as_ref().map(|s| &**s),
                                        r.title.as_ref().map(|s| &**s),
                                        r.artist.as_ref().map(|s| &**s),
                                        None,
                                        r.length.as_ref().map(|s| &**s),
                                    ));
                                }
                            }
                        }

                        if tracks.len() > 0 {
                            out.push(TabData {
                                rows: tracks,
                                uuid: section.to_string(),
                                label,
                                position,
                                selected: &selected_tab == section,
                            });
                            continue;
                        }
                    }
                }
                self.config.remove_section(section);
                need_save = true;
            }
        }

        if need_save {
            self.save();
        }

        out.sort_by_key(|tab| tab.position);

        out
    }
}
