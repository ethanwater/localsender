use super::config::get_storage;
use super::utils;
use crate::soi::packet::Packet;
use bincode;
use std::fs::{self};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::path;
use std::path::Path;
use std::time::Instant;

pub async fn build() -> std::io::Result<Soi> {
    let soi_instance = Soi {
        //storage_location: String::from(""),
        //objects: 0,
    };
    return Ok(soi_instance);
}
pub struct Soi {}

impl Soi {
    pub async fn launch(&mut self) -> std::io::Result<()> {
        let listener = fetch_listener().await.unwrap();
        dbg!(listener.local_addr()?);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    tokio::spawn(async move {
                        let storage = get_storage().await.unwrap();
                        process_packet(stream, storage).await;
                    });
                }
                Err(_) => {
                    println!("fuck");
                }
            }
        }
        Ok(())
    }
}

async fn process_packet(mut stream: std::net::TcpStream, storage: String) {
    loop {
        let mut buf = [0; 1024];
        let now = Instant::now();

        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                //buf.truncate(n);
                let packet: Result<Packet, bincode::Error> = bincode::deserialize_from(&buf[..]);
                dbg!(&packet);
                match packet {
                    Ok(packet) => {
                        let uploaded_file_path = storage.clone() + &packet.filename;
                        dbg!(&uploaded_file_path);

                        match packet.command.as_str() {
                            "upload--force" => {
                                println!(
                                    "🍜 soi | retrieved: {:?} [size: {:?} bytes]",
                                    packet.filename, packet.size
                                );
                                fs::write(&uploaded_file_path, &packet.data)
                                    .unwrap_or_else(|e| eprintln!("Error writing file: {:?}", e));
                                println!("🍜 soi | {:?} has been overwritten", packet.filename);
                            }
                            "upload" => {
                                println!(
                                    "🍜 soi | retrieved: {:?} [size: {:?} bytes]",
                                    packet.filename, packet.size
                                );
                                if !path::Path::exists(Path::new(&uploaded_file_path)) {
                                    fs::write(&uploaded_file_path, &packet.data).unwrap_or_else(
                                        |e| eprintln!("Error writing file: {:?}", e),
                                    );
                                    println!("🍜 soi | File uploaded: {:?}", packet.filename);
                                } else {
                                    println!(
                                        "🍜 soi | {:?} already exists, will not write shipped file",
                                        packet.filename
                                    );
                                }
                            }
                            "download" => {
                                println!(
                                    "🍜 soi | retrieved request to send: {} to {:?}",
                                    packet.filename,
                                    stream.peer_addr().unwrap()
                                );
                                let bytes = fs::read(&uploaded_file_path).unwrap();
                                dbg!(&bytes.len());

                                match stream.write_all(&bytes) {
                                    Ok(_) => println!(
                                        "🍜 soi | sent {} to {:?}: {:?}μs",
                                        packet.filename,
                                        stream.peer_addr().unwrap(),
                                        now.elapsed().as_micros()
                                    ),
                                    Err(e) => eprintln!("Error sending data: {:?}", e),
                                }
                            }
                            &_ => {
                                eprintln!("Unknown command: {:?}", packet.command);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error deserializing packet: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from stream: {:?}", e);
                break;
            }
        }
    }
}

async fn fetch_listener() -> std::io::Result<std::net::TcpListener> {
    let socket_addr: SocketAddr =
        utils::retrieve_local_socket_addr().expect("🍜 soi | unable to obtain address");

    if let Ok(listener) = std::net::TcpListener::bind(socket_addr) {
        println!("🍜 soi | found {:?}", listener.local_addr().unwrap());
        return Ok(listener);
    } else {
        println!("🍜 soi | unable to connect to the port, searching for others...");
        if let Ok(listener) = std::net::TcpListener::bind("127.0.0.1:0") {
            println!("🍜 soi | found {:?}", listener.local_addr());
            return Ok(listener);
        };
        return Err(std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "🍜 soi | unable to fetch listener",
        ));
    }
}
