#![allow(dead_code)]
#![allow(unused)]

use super::config;
use super::utils;
use crate::soi::packet::Packet;
use bincode;
use std::fs::{self};
use std::io::{Read, Write};
use std::net::{self, SocketAddr};
use std::path;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

pub async fn build() -> std::io::Result<Soi> {
    let soi_instance = Soi {
        storage_location: String::from(""),
        storage_used: 0,
        objects: 0,
    };
    return Ok(soi_instance);
}
pub struct Soi {
    storage_location: String,
    storage_used: usize,
    objects: usize,
}

impl Soi {
    async fn calc_storage_used(&mut self) {
        let storage = fs::read_dir(Path::new(&self.storage_location))
            .expect("🍜 soi | storage location invalid");

        for file in storage {
            if file.is_ok() {
                let metadata = file.unwrap().metadata().unwrap();
                if metadata.is_file() {
                    self.storage_used += metadata.len() as usize;
                }
                if metadata.is_dir() {
                    todo!();
                }
                self.objects += 1;
            }
        }
    }

    pub async fn set_storage(&mut self) {
        let storage_path = config::soi_config().unwrap();
        if Path::exists(Path::new(storage_path.as_str())) {
            self.storage_location = storage_path;
            return;
        }
        println!("🍜 soi | {storage_path} does not exist");
    }

    pub async fn launch(&mut self) -> std::io::Result<()> {
        self.set_storage().await;
        self.calc_storage_used().await;

        let listener = fetch_listener().await?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    tokio::spawn(async move {
                        process_packet(stream).await;
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

async fn fetch_listener() -> std::io::Result<std::net::TcpListener> {
    let socket_addr: SocketAddr =
        utils::retrieve_local_socket_addr().expect("🍜 soi | unable to obtain address");

    if let Ok(listener) = std::net::TcpListener::bind(socket_addr) {
        return Ok(listener);
    } else {
        println!("🍜 soi | unable to connect to the port, searching for others...");
        //questionable, i dont know if this should be the else case...
        //cause why df host on a random port? instead we can recommend it:
        if let Ok(listener) = std::net::TcpListener::bind("127.0.0.1:0") {
            println!("🍜 soi | found {:?}", listener.local_addr());
            //TODO: some boring shit on wether the user wants to use this port or not (y or n)
            return Ok(listener);
        };
        return Err(std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "🍜 soi | unable to fetch listener",
        ));
    }
}

async fn process_packet(mut stream: std::net::TcpStream) {
    loop {
        let mut buf = vec![0; 1024]; // Initialize buffer with a size

        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    // End of stream, break the loop
                    break;
                }

                // Trim the buffer to the actual number of bytes read
                buf.truncate(n);

                // Deserialize the packet
                let packet: Result<Packet, bincode::Error> = bincode::deserialize_from(&buf[..]);
                match packet {
                    Ok(packet) => {
                        let storage = "/Users/ethan/.config/soistorage/".to_string();
                        let uploaded_file_path = storage + &packet.filename;

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
                                    fs::write(&uploaded_file_path, &packet.data)
                                        .unwrap_or_else(|e| eprintln!("Error writing file: {:?}", e));
                                    println!("🍜 soi | File uploaded: {:?}", packet.filename);
                                } else {
                                    println!("🍜 soi | {:?} already exists, will not write shipped file", packet.filename);
                                }
                            }
                            "download" => {
                                println!(
                                    "🍜 soi | retrieved request to send: {:?} to {:?}",
                                    packet.filename, stream.peer_addr().unwrap()
                                );
                                let bytes = fs::read(&packet.filename).unwrap();

                                // Sending the file data
                                match stream.write_all(&bytes) {
                                    Ok(_) => println!("🍜 soi | Sent file data"),
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
