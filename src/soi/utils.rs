use std::net::{IpAddr, SocketAddr};
use local_ip_address::local_ip;

pub fn retrieve_socket_addr() -> std::io::Result<SocketAddr> {
    let local_ip: IpAddr = local_ip().unwrap();
    if local_ip.is_ipv4() {
        let default = SocketAddr::new(local_ip.to_canonical(), 8080);
        return Ok(default);
    }

    return Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "🍜 soi | failed to retrieve IP address"))
}

pub fn obtain_bytes(filepath: &str) -> std::io::Result<(Vec<u8>, usize)> {
    let bytes = match std::fs::read(filepath) {
        Ok(bytes) => bytes,
        Err(e) => return Err(e),
    };
    let size = &bytes.len();
    Ok((bytes, *size))
}