use serde::{Deserialize, Serialize};
use std::fs;

// Take a PathBuf and return a result with a Config struct if its ok
pub fn load_config(fp: std::path::PathBuf) -> Result<Config, ()> {
    if let Ok(content) = fs::read_to_string(fp) {
        // TODO: There is a chance that we panic if content is not actually a tomly file
        // Remove unwrap later
        let config: Config = toml::from_str(&content).unwrap();
        return Ok(config);
    } else {
        return Err(());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    colors: Option<Colors>,
    cursor: Option<Cursor>,
    font: Option<Font>,
    selection: Option<Selection>,
    window: Option<Window>,
}

// Level 1
#[derive(Serialize, Deserialize, Debug)]
pub struct Colors {
    primary: Option<Primary>,
    selection: Option<Selection>,
    cursor: Option<Cursor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cursor {
    blink_interval: Option<i32>,
    blink_timeout: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Font {
    size: Option<i32>,
    normal: Option<FontDef>,
    bold: Option<FontDef>,
    italic: Option<FontDef>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Selection {
    background: Option<String>,
    text: Option<String>,
    save_to_clipboard: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Window {
    opacity: Option<f32>,
}

// Level 2
#[derive(Serialize, Deserialize, Debug)]
pub struct Primary {
    background: Option<String>,
    foreground: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FontDef {
    family: Option<String>,
    style: Option<String>, // Change this to enum later
}
