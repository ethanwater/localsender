#![allow(unused)]

use super::packet;
use crate::soi::packet::Packet;
use bincode;
use std::fs::{self};
use std::io::{Read, Write};
use std::net::{self, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub struct Soi {
    storage_volume: String,
    storage_max: usize,
    storage_available: usize,
    storage_used: usize,
    port: u16,
    addr: net::IpAddr,
    listener: net::TcpListener,
    objects: usize,
}

pub fn build() -> std::io::Result<Soi> {
    if let Ok(fetched_listener) = fetch_listener() {
        let soi_instance = Soi {
            storage_volume: String::from(""),
            storage_max: std::usize::MAX,
            storage_available: 0,
            storage_used: 0,
            port: fetched_listener.local_addr()?.port(),
            addr: fetched_listener.local_addr()?.ip(),
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

impl Soi {
    fn storage_update(&mut self) {
        self.storage_available = self.storage_max - self.storage_used
    }

    fn storage_fetch_volume_size(&mut self) {
        todo!()
    }

    pub fn launch(&mut self) -> std::io::Result<()> {
        let mut listener = self
            .listener
            .try_clone()
            .expect("🍜 soi | failed to initialize handle");

        let lock: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));

        for stream in listener.incoming() {
            let lock2 = Arc::clone(&lock);
            if stream.is_ok() {
                process_packet(stream.unwrap(), lock2);
            }
        }

        Ok(())
    }
}

fn fetch_listener() -> std::io::Result<net::TcpListener> {
    let listener: TcpListener =
        net::TcpListener::bind("127.0.0.1:8080").expect("🍜 soi | unable to find available port");
    let address: SocketAddr = listener.local_addr()?;

    println!("🍜 | soi hosting on {address}");
    Ok(listener)
}

fn process_packet(mut stream: TcpStream, lock: Arc<Mutex<u8>>) {
    //todo: make sure the file does not already exist. if it does, it requires a force shipment
    //from the client
    let _ = std::thread::spawn(move || {
        let mut contents: Vec<u8> = Vec::new();
        stream
            .read_to_end(&mut contents)
            .expect("🍜 soi | failed to read data");

        let packet: Packet = bincode::deserialize_from(&*contents)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            .expect("shit");

        match packet.command.as_str() {
            "upload" => {
                let _guard = lock.lock().unwrap();

                println!(
                    "🍜 | soi retrieved: {:?} [size: {:?} bytes]",
                    packet.filename, packet.size
                );
                fs::write(&packet.filename, &packet.data).unwrap();
            }
            "download" => {
                println!(
                    "🍜 | soi retrieved request to send: {:?}",
                    packet.filename
                );
                let bytes = fs::read(&packet.filename).unwrap();

                stream.write_all(&bytes).expect("shit");
                println!("dibe");
            }
            &_ => todo!(),
        }
    })
    .join();
}

