use std::fs;
use std::io::BufRead;

use super::utils;

pub fn soi_config() -> std::io::Result<String> {
    let config: Vec<u8>;
    let username = utils::username_unix().unwrap();

    match std::env::consts::OS {
        "linux" => {
            let path = "/home/".to_owned() + username.as_str() + "/.config/soiconfig";
            config = fs::read(path).expect("🍜 soi | configuration file does not exist");
        }
        "macos" => {
            let path = "/Users/".to_owned() + username.as_str() + "/.config/soiconfig";
            config = fs::read(path).expect("🍜 soi | configuration file does not exist");
        }
        &_ => todo!(),
    }

    let storage_path = config
        .lines()
        .nth(0)
        .expect("🍜 soi | unable to read configuration file")
        .unwrap();

    if !std::path::PathBuf::from(&storage_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "🍜 soi | the providded storage path in the configuration file does not exist",
        ));
    };
    return Ok(storage_path);
}

pub fn set_storage(storage_path: &str) -> std::io::Result<()> {
    let username = utils::username_unix().unwrap();
    match std::env::consts::OS {
        "linux" => {
            let path = "/home/".to_owned() + username.as_str() + "/.config/soiconfig";
            if let Ok(_) = fs::write(path, storage_path) {
                return Ok(());
            }
        }
        "macos" => {
            let path = "/Users/".to_owned() + username.as_str() + "/.config/soiconfig";
            if let Ok(_) = fs::write(path, storage_path) {
                return Ok(());
            }
        }
        &_ => todo!(),
    }

    return Err(std::io::Error::new(
        std::io::ErrorKind::WriteZero,
        "🍜 soi | unable to set storage path",
    ));
}
