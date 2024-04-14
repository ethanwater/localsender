use local_ip_address::local_ip;
use std::fs;
use std::io::BufRead;
use std::net::{IpAddr, SocketAddr};

pub fn retrieve_socket_addr() -> std::io::Result<SocketAddr> {
    let local_ip: IpAddr = local_ip().expect("🍜 soi | failed to retrieve IP address");
    if local_ip.is_ipv4() {
        let default = SocketAddr::new(local_ip.to_canonical(), 8080);
        return Ok(default);
    }

    return Err(std::io::Error::new(
        std::io::ErrorKind::AddrNotAvailable,
        "🍜 soi | failed to retrieve IP address",
    ));
}

pub fn obtain_bytes(filepath: &str) -> std::io::Result<(Vec<u8>, usize)> {
    let bytes = match fs::read(filepath) {
        Ok(bytes) => bytes,
        Err(e) => return Err(e),
    };
    let size = &bytes.len();
    Ok((bytes, *size))
}

pub fn soi_config() -> String {
    let config = fs::read("/home/ethan/.soiconfig").unwrap(); //ill change this dw
    let storage_path = config.lines().nth(0).unwrap().unwrap(); //this is mega cancer, but DONT FREAK OUT- placeholder til i have something to parse with. i hate parsing, its the most boring part.

    return storage_path;
}
