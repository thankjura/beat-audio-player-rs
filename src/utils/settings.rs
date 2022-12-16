use std::{path::PathBuf, fs, io};
use std::fs::File;
use std::io::BufRead;
use gtk::glib;
use configparser::ini::Ini;
use lazy_static::lazy_static;
use regex::Regex;

use crate::structs::track::Track;

pub struct BeatSettings {
    config: Ini,
    config_dir: PathBuf,
    path: PathBuf,
}

lazy_static! {
    static ref REGEXP: Regex = Regex::new(r"^[a-f0-9]{8}-?[a-f0-9]{4}-?4[a-f0-9]{3}-?[89ab][a-f0-9]{3}-?[a-f0-9]{12}").unwrap();
}

#[derive(Debug)]
pub struct TabData {
    pub rows: Vec<Track>,
    pub uuid: String,
    pub label: String,
    position: u32,
    pub selected: bool,
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
        self.config.write(&self.path).expect("Can't save configuration");
    }

    pub fn playlists(&mut self) -> Vec<TabData> {
        let mut out = vec![];
        let mut need_save = false;

        let selected_tab = self.config.get("main", "selected").unwrap_or("000".to_string());

        for section in self.config.sections().iter() {
            if REGEXP.is_match(section) {
                let mut section_map = self.config.get_map_ref()[section].clone();

                if let Some(Some(playlist_path)) = &section_map.get("file").clone() {
                    let playlist_path = PathBuf::from(self.config_dir.join(playlist_path));
                    if let Ok(file) = File::open(&playlist_path) {
                        let lines = io::BufReader::new(file).lines();
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
                            for r in reader.records() {
                                if let Ok(r) = r {
                                    let path = PathBuf::from(r.get(0).unwrap());
                                    tracks.push(Track::new(
                                        path.file_name().unwrap().to_str().unwrap(),
                                        path.to_str().unwrap(),
                                        r.get(2),
                                        r.get(3),
                                        r.get(1),
                                        None,
                                        r.get(4),
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
                                selected: &selected_tab == section
                            });
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

        out.sort_by_key(|tab| {tab.position});

        out
    }
}