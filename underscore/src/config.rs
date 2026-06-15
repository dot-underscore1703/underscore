use std::path::PathBuf;
use std::fs;
use dirs;
use toml;
use serde::Deserialize;

// Personally, coding stuff to add support for config is my favorite.
// There's just something about giving the user freedom that I enjoy.

// This function gets the users '.config' directory and appends 'underscore' to it. If that path doesnt exist it creates it.
fn get_underscore_cfg_dir() -> std::io::Result<PathBuf> {
    let mut cfg_dir = dirs::config_dir().expect("Could not get '.config' directory!");
    cfg_dir.push("underscore");

    fs::create_dir_all(&cfg_dir).ok();
    Ok(cfg_dir)
}

#[derive(Deserialize)]
pub struct Config {
    pub text: Text,

    pub fade: Fade,

    pub colors: Colors,

    pub padding: Padding

}

#[derive(Deserialize)]
pub struct Colors {
    color_fg: String,
    color_bg: String,
    color_text: String
}

// so users can pick if their fade is low-taper or not.
#[derive(Deserialize)]
pub struct Fade {
    pub fade_in_ms: u16,
    pub fade_out_ms: u16,
}

#[derive(Deserialize)]
pub struct Text {
    font: String,
    size: u16,
}

#[derive(Deserialize)]
pub struct Padding {
    pub margin_top: u16,
    pub margin_bottom: u16,
    pub margin_start: u16,
    pub margin_end: u16
}

pub fn get_config_from_file() -> Config {
    let mut cfg_path = get_underscore_cfg_dir()
        .expect("Could not get '~/.config/underscore/' directory!");
    cfg_path.push("config.toml");

    let cfg_contents = &fs::read_to_string(cfg_path).expect("Could not read toml!");

    toml::from_str(cfg_contents).expect("Could not parse toml!")
}