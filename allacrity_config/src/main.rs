pub mod config;
use config::Config;
use dirs::home_dir;

fn main() {
    // 
    let mut something: Result<Config, ()> = Err(());
    if let Some(mut path) = home_dir() {
        println!("{:?}", &path);
        path.as_mut_os_string().push("/.alacritty.toml");
        something = config::load_config(path);
    } else {
        println!("DIED")
    }

    if something.is_ok() {
        println!("{:#?}", something.unwrap());
    } else {
        println!("you suck");
    }

    let home = home_dir();
    
}
