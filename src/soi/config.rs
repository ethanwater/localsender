use std::fs;
use std::io::BufRead;

pub fn soi_config() -> String {
    let config = fs::read(".soiconfig").expect("🍜 soi | configuration file does not exist"); 
    let storage_path = config.lines().nth(0).expect("🍜 soi | unable to read configuration file").unwrap(); 
    return storage_path;
}

pub fn set_storage(storage_path: &str) -> std::io::Result<()> {
    if let Ok(_) = fs::write(".soiconfig", storage_path) {
        return Ok(());
    }
    return Err(std::io::Error::new(
        std::io::ErrorKind::WriteZero,
        "🍜 soi | unable to set storage path",
    ));
}
