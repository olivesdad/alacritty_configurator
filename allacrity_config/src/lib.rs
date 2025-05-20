use std::fs;
use config::{Config, Primary};
mod config;

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

//impl Config {
//    pub fn change_bg_color<T>(color: T) -> Result<(),()>{
//        match color {
//            Primary {background: bg, fg}=>{
//
//            },
//        }
//
//        Ok(())
//    }
//}