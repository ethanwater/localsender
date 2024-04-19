#![allow(dead_code)]

use super::config;
use super::utils;
use crate::soi::packet::Packet;
use bincode;
use std::fs::{self};
use std::net::{self, SocketAddr};
use std::path;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

pub async fn build() -> std::io::Result<Soi> {
    if let Ok(fetched_listener) = fetch_listener().await {
        let soi_instance = Soi {
            storage_location: String::from(""),
            storage_used: 0,
            addr: fetched_listener.local_addr()?,
            listener: fetched_listener,
            objects: 0,
        };
        return Ok(soi_instance);
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "🍜 soi | failed to fetch listener",
        ))
    }
}
pub struct Soi {
    storage_location: String,
    storage_used: usize,
    addr: net::SocketAddr,
    listener: tokio::net::TcpListener,
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

    pub async fn set_addr(&mut self, addr: &str) {
        self.listener = TcpListener::bind(addr)
            .await
            .expect("🍜 soi | unable to bind to provided address");
        self.addr = self.listener.local_addr().unwrap();
    }

    pub async fn launch(&mut self) -> std::io::Result<()> {
        self.set_storage().await;
        self.calc_storage_used().await;

        let listener = &self.listener;

        println!(
            "🍜 soi server configuration\n    host:    {}\n    storage: {}",
            self.addr, self.storage_location
        );
        let storage = Arc::new(self.storage_location);

        loop {
            let (stream, _addr) = listener.accept().await?;

            tokio::spawn(async move {
                process_packet(stream, storage.clone().to_string()).await;
            });
        }
    }
}

async fn fetch_listener() -> std::io::Result<tokio::net::TcpListener> {
    let socket_addr: SocketAddr =
        utils::retrieve_local_socket_addr().expect("🍜 soi | unable to obtain address");

    if let Ok(listener) = tokio::net::TcpListener::bind(socket_addr).await {
        return Ok(listener);
    } else {
        println!("🍜 soi | unable to connect to the port, searching for others...");
        //questionable, i dont know if this should be the else case...
        //cause why df host on a random port? instead we can recommend it:
        if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:0").await {
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

async fn process_packet(stream: TcpStream,  storage: String) {
    let mut contents: Vec<u8> = vec![];

    let mut stream = stream;
    stream
        .read_to_end(&mut contents)
        .await
        .expect("🍜 soi | failed to read data");

    let packet: Packet = bincode::deserialize_from(&*contents)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        .expect("🍜 soi | shit...");

    let uploaded_file_path = storage + packet.filename.as_str();

    match packet.command.as_str() {
        "upload--force" => {
            println!(
                "🍜 soi | retrieved: {:?} [size: {:?} bytes]",
                packet.filename, packet.size
            );
            fs::write(&uploaded_file_path, &packet.data).unwrap();
            println!("🍜 soi | {:?} has been overwritten", packet.filename);
        }
        "upload" => {
            println!(
                "🍜 soi | retrieved: {:?} [size: {:?} bytes]",
                packet.filename, packet.size
            );
            if !path::Path::exists(Path::new(&uploaded_file_path)) {
                fs::write(&uploaded_file_path, &packet.data).unwrap();
                return;
            }
            println!(
                "🍜 soi | {:?} already exists, will not write shipped file",
                packet.filename
            );
        }
        "download" => {
            println!(
                "🍜 soi | retrieved request to send: {:?} to {:?}",
                packet.filename,
                stream.peer_addr().unwrap()
            );
            //let bytes = fs::read(&packet.filename).unwrap();
            //dbg!(&bytes);

            //stream.write_all(&bytes).await.expect("🍜 soi | shit...");
        }
        &_ => todo!(),
    }
    std::mem::drop(packet);
}
