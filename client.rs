#![allow(unused)]

use super::object;
use bincode;
use core::time;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fs::{self, File};
use std::io::{BufReader, IoSlice, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::thread;

const RETRY_COUNT: u8 = 5;

type ClientCommand = String;

pub fn upload_unix(host: &str, filepath: &str, mut attempts: u8) -> std::io::Result<()> {
    //upload_unix() works under the assumption that both devices share similar endianess:
    //  -macOS uses ARM64  x86_64-based hardware: LITTLE ENDIAN
    //  -Most Linux systems today run on x86, x86_64, and ARM architectures, which are all little-endian by default.

    if let Ok(mut stream) = TcpStream::connect(host) {
        let filepath_buffer = PathBuf::from(filepath);
        match filepath_buffer.try_exists() {
            Ok(exists) => {
                if !exists {
                    println!("🍜 soi | the path does not exist.");
                    return Ok(());
                }
            },
            Err(error) => {
                println!("🍜 soi | failure checking the path: {:?}", error);
                return Err(error);
            },
        } 

        let dataset = self::obtain_bytes(filepath)?;

        let filename = String::from(
            filepath_buffer
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or(filepath),
        );

        let cmd: ClientCommand = String::from("upload");

        let object = object::Packet {
            command: cmd,
            filename: filename.clone(),
            data: dataset.0,
            size: dataset.1,
        };

        if let Ok(packet) = bincode::serialize(&object) {
            stream
                .write(&packet)
                .expect("🍜 soi | failed to ship to host");
            println!("🍜 soi | {filename} shipped to {host}");
        };

        std::mem::drop(object);
    } else {
        println!("🍜 soi | failed to connect to host, trying again in 3 seconds...");
        if attempts >= RETRY_COUNT {
            println!("🍜 soi | lmao rip");
            return Ok(());
        }
        thread::sleep(time::Duration::from_secs(3));
        attempts += 1;
        upload_unix(host, filepath, attempts);
    }

    Ok(())
}

fn obtain_bytes(filepath: &str) -> std::io::Result<(Vec<u8>, usize)> {
    let bytes = match fs::read(filepath) {
        Ok(bytes) => bytes,
        Err(e) => return Err(e),
    };
    let size = &bytes.len();
    Ok((bytes, *size))
}

fn detect_soi_instance() -> std::io::Result<()> {
    todo!();
    Ok(())
}

fn download(filename: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(filename).expect("🍜 soi | failed to create file");
    file.write_all(data);
    Ok(())
}
