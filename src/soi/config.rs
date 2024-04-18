use std::fs;
use std::io::BufRead;

pub fn soi_config() -> String {
    let config: Vec<u8>;
    match std::env::consts::OS {
        //TODO: replace 'ethan' with computer username
        "linux" => {
            config = fs::read("/home/ethan/.soiconfig")
                .expect("🍜 soi | configuration file does not exist");
        }
        "macos" => {
            todo!();
        }
        "windows" => {
            todo!();
        }
        &_ => todo!(),
    }

    let storage_path = config
        .lines()
        .nth(0)
        .expect("🍜 soi | unable to read configuration file")
        .unwrap();
    return storage_path;
}

pub fn set_storage(storage_path: &str) -> std::io::Result<()> {
    match std::env::consts::OS {
        "linux" => {
            if let Ok(_) = fs::write("/home/ethan/.soiconfig", storage_path) {
                return Ok(());
            }
        }
        "macos" => {
            todo!();
        }
        "windows" => {
            todo!();
        }
        &_ => todo!(),
    }

    return Err(std::io::Error::new(
        std::io::ErrorKind::WriteZero,
        "🍜 soi | unable to set storage path",
    ));
}
