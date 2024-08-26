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
    scrolling: Option<Scrolling>,
    bell: Option<Bell>,
    terminal: Option<Terminal>,
    mouse: Option<Mouse>,
    hint: Option<Hint>,
    keyboard: Option<Keyboard>,
    debug: Option<Debug>,
    font: Option<Font>,
    // selection is a shared struct
    selection: Option<Selection>,
    window: Option<Window>,
}

// Level 1

// TODO These ones
#[derive(Serialize, Deserialize, Debug)]
pub struct Scrolling {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bell {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Terminal {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mouse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hint {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keyboard {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Debug {}

// DONE //
#[derive(Serialize, Deserialize, Debug)]
pub struct Colors {
    primary: Option<Primary>,
    cursor: Option<Cursor>,
    vi_mode_cursor: Option<ViModeCursor>,
    search: Option<Search>,
    hints: Option<Hints>,
    line_indicator: Option<LineIndicator>,
    footer_bar: Option<FooterBar>,
    selection: Option<Selection>,
    bright: Option<Bright>,
    dim: Option<Dim>,
    indexed_colors: Option<IndexedColors>,
    transparent_background_colors: Option<TransparentBgColors>,
    draw_bold_text_with_bright_colors: Option<DrawBoldText>,
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

// TODO
/* 

 */

 #[derive(Serialize, Deserialize, Debug)]
pub struct ViModeCursor {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Search {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Hints {}
#[derive(Serialize, Deserialize, Debug)]
pub struct LineIndicator {}
#[derive(Serialize, Deserialize, Debug)]
pub struct FooterBar {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Normal {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Bright {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Dim {}
#[derive(Serialize, Deserialize, Debug)]
pub struct IndexedColors {}
#[derive(Serialize, Deserialize, Debug)]
pub struct TransparentBgColors {}
#[derive(Serialize, Deserialize, Debug)]
pub struct DrawBoldText {}



// Shared Structs

// Level 1 and level 2 for Colors
#[derive(Serialize, Deserialize, Debug)]
pub struct Selection {
    background: Option<String>,
    text: Option<String>,
    save_to_clipboard: Option<bool>,
}