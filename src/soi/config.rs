use std::fs;
use std::io::BufRead;

pub fn soi_config() -> String {
    let config = fs::read("/home/ethan/.soiconfig").unwrap(); //ill change this dw
    let storage_path = config.lines().nth(0).unwrap().unwrap(); //this is mega cancer, but DONT FREAK OUT- placeholder til i have something to parse with. i hate parsing, its the most boring part.

    return storage_path;
}

pub fn set_storage(storage_path: &str) -> std::io::Result<()> {
    if let Ok(_) = fs::write("/home/ethan/.soiconfig", storage_path) {
        return Ok(());
    }
    return Err(std::io::Error::new(
        std::io::ErrorKind::WriteZero,
        "🍜 soi | unable to set storage path",
    ));
}
