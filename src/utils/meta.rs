use std::fs;
use std::io::Write;
use std::path::PathBuf;
use gettextrs::gettext;
use gtk::glib;
use lofty::{Accessor, AudioFile, ItemKey};
use crate::structs::track::Track;

fn get_cache_dir() -> PathBuf {
    glib::user_cache_dir().join("beat").join("covers")
}

fn get_tag(filepath: &str) -> Option<lofty::Tag> {
    if let Ok(tagged_file) = lofty::read_from_path(filepath) {
        return match tagged_file.primary_tag() {
            Some(primary_tag) => { Some(primary_tag.clone()) }
            None => { tagged_file.first_tag().as_deref().cloned() }
        };
    }

    None
}

pub fn get_album_picture_path(path: &str) -> Option<PathBuf> {
    let hash = format!("{:x}", md5::compute(path));
    let cache_dir = get_cache_dir();
    let cover_path = cache_dir.join(hash);
    if cover_path.is_file() {
        return Some(cover_path);
    }

    if let Some(tag) = get_tag(path) {
        let pictures = tag.pictures();
        if let Some(pic) = pictures.get(0) {
            if !cache_dir.exists() {
                fs::create_dir_all(cache_dir);
            }
            if let Ok(mut file) = fs::File::create(&cover_path) {
                file.write_all(pic.data());
                return Some(cover_path);
            } else {
                println!("{}", gettext("Can't create cover cache"));
            }
        }
    }


    None
}

pub fn get_track_from_path(filepath: &str) -> Option<Track> {
    let filepath = PathBuf::from(filepath);
    if !filepath.is_file() {
        return None;
    }

    let filename = filepath.file_name().unwrap().to_str().unwrap();

    if let Ok(tagged_file) = lofty::Probe::open(&filepath).unwrap().read() {
        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => Some(primary_tag),
            None => tagged_file.first_tag(),
        };

        if let Some(tag) = tag {
            let properties = tagged_file.properties();
            let duration = properties.duration();
            return Some(Track::new(
                filename,
                filepath.to_str().unwrap(),
                tag.get_string(&ItemKey::AlbumTitle),
                tag.get_string(&ItemKey::TrackTitle),
                tag.get_string(&ItemKey::TrackArtist),
                tag.year(),
                Some(duration.as_secs())
            ));
        };
    }


    Some(Track::new(
        filename,
        filepath.to_str().unwrap(),
        None,
        None,
        None,
        None,
        None
    ))
}