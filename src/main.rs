extern crate wallpaper;
use chrono::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::{thread, time};
use toml::from_str;

#[derive(Debug, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Wallpaper {
    path: String,
    hour: u32,
    min: u32,
}

impl Wallpaper {
    pub fn new(path: String) -> Self {
        Wallpaper {
            path,
            hour: 1,
            min: 0,
        }
    }
    pub fn set_time(&mut self, hour: u32, min: u32) {
        self.hour = hour;
        self.min = min;
    }
}

fn main() {
    let mut auto_mode = false;
    let mut verbose_mode = false;
    for argument in env::args() {
        if argument == "auto" {
            auto_mode = true;
        } else if argument == "verbose" {
            verbose_mode = true;
        }
    }

    let mut vec: Vec<Wallpaper>;

    if auto_mode {
        vec = get_files(Path::new("wallpapers")).expect("files not found");
        wallpapers_time_set(&mut vec);
    } else {
    let key = "CONFIG_MURPAPIER";
        match get_wallpaper_from_config()
        {
            Ok(v) => vec = v,
            Err(e) => panic!("{} {}",e, env::var(key).unwrap())
        }
        vec.sort_by(|a, b| (a.hour * 60 + a.min).cmp(&(b.hour * 60 + b.min)));
    }
    if verbose_mode {
        println!("auto_mode: {}", auto_mode);

        for k in &vec {
            println!("Wallpaper :{} scheduled at {}:{}", k.path, k.hour, k.min);
        }
    }

    loop {
        let local: DateTime<Local> = Local::now();

        for wp in &vec {
            if wp.hour == local.time().hour() && wp.min == local.time().minute() {
                if let Err(val) = wallpaper::set_from_path(&wp.path) {
                    println!(
                        "{} issue at {}:{} with {} file",
                        val, wp.hour, wp.min, wp.path
                    );
                }
                if verbose_mode {
                    println!("Changed to {}", wp.path);
                }
                break;
            }
        }
        let sleep_time = time::Duration::from_secs(60);
        thread::sleep(sleep_time);
    }
}

/// Returns the files read from a given directory
fn get_files(dir: &Path) -> Result<Vec<Wallpaper>, io::Error> {
    let mut res: Vec<Wallpaper> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let absolute_path = fs::canonicalize(entry.path())?;
            let correct_path: String = match absolute_path.to_str() {
                None => panic!("Unable to get the absolute path of a file"),
                Some(path) => String::from(path),
            };

            let wp = Wallpaper::new(correct_path);
            res.push(wp);
        }
    }
    res.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(res)
}

fn wallpapers_time_set(wallpapers: &mut Vec<Wallpaper>) {
    let lenght = wallpapers.len();

    let mut index = 0;
    for wallpaper in wallpapers {
        let (hours, minutes) = to_hour_minutes(index, lenght as i32);
        wallpaper.set_time(hours as u32, minutes as u32);
        index += 1;
    }
}

/// Get the hour and minutes needed for a set of wallpaper with an index
fn to_hour_minutes(index: i32, lenght: i32) -> (i32, i32) {
    let mut minutes: f32 = (24.0 / lenght as f32) * 60.0;
    minutes *= index as f32;
    let hours: i32 = minutes as i32 / 60;
    minutes %= 60.0;

    (hours, minutes as i32)
}

fn get_wallpaper_from_config() -> Result<Vec<Wallpaper>, io::Error> {
    let mut path: PathBuf = PathBuf::new();
    let key_dir ="WALLPAPER_DIR";

    if let Ok(dir) = env::current_dir() {
        path.push(dir);
    }
    let key = "CONFIG_MURPAPIER";

    let config_path = env::var(key).expect("CONFIG_MURPAPIER env variable must be set to the config.toml file path");
    println!("{}", config_path);
    let config: String = fs::read_to_string(Path::new(&config_path))?;

    let items_table: HashMap<String, Vec<Wallpaper>> =
        from_str(&config).expect("Can't file config.toml");
    let items: &[Wallpaper] = &items_table["image"];

    let mut vec = items.to_vec();

    let dir_path = env::var(key_dir).expect("WALLPAPER_DIR env var must be set to the wallpapers folder");
    println!("{}", dir_path);
    let wall_path = Path::new(&dir_path);
    for k in &mut vec {
        if let Some(p) = wall_path.to_str() {
            k.path = p.to_owned() + "/" + &k.path;
        }
    }

    Ok(vec)
}
