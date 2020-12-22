extern crate dirs;

use dirs::config_dir;
use std::fs;
use std::path::PathBuf;

// This whole thing basically copies dicts to your config directory
// I know. It's long.
fn main() -> std::io::Result<()> {
    match config_dir() {
        Some(config_dir) => {
            let mut config_dir = config_dir;
            config_dir.push(PathBuf::from("numatim/dicts/"));
            fs::create_dir_all(&config_dir)?;

            // this is REALLY stupid but I have no idea what's more appropriate than this
            config_dir.push(PathBuf::from("json"));

            for file in fs::read_dir("dicts")? {
                let file = file?.path();

                if file.is_file() {
                    let mut dst = PathBuf::from(&config_dir);
                    let fl = file.file_name().unwrap();

                    dst.set_file_name(fl);
                    fs::copy(&file, dst)?;
                }
            }
        }
        None => {
            eprintln!("You are required to have a config directory to use this program. Aborting!")
        }
    };
    println!("cargo:rerun-if-changed=dicts");
    Ok(())
}
