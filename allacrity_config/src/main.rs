pub mod config;
use std::process::exit;
use std::fs;

use dirs::home_dir;

fn main() {

    let home = home_dir();
    let mut config_file = match home {
        Some(mut path) =>{
            path.push(".alacritty.toml");
            println!("attempting to load config file from: {:?}", &path);
            config::load_config(path)
            },
        None => Err(())
    };


    if config_file.is_ok() {
        println!("Succesfully loaded config file");
     //  println!("{:#?}", config_file.unwrap());
    } else {
        exit(1);
    }
    let mut cf = config_file.unwrap();
    //try to change color 
   // let cf.colors.unwrap().primary.unwrap().background = Some(String::from("000000"));
    if let Some(ref mut colors) = cf.colors {
        if let Some(ref mut pirmary) = colors.primary {
            pirmary.background = Some(String::from("000000"));
        }
    }
    println!("{:#?}", cf);
    let out = toml::to_string(&cf).unwrap();
    println!("{:#?}", out);
    // write to file
    let andy = home_dir();
        if let Some(mut path) = andy {
            path.push("boogers.txt");
            fs::write(path.into_os_string(), out).unwrap();
        }
    //fs::write(andy, out).unwrap();
    //fs::write("~/boogers.txt", out).unwrap();
    println!("HIHI")
    
}
