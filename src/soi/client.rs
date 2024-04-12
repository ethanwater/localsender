#![allow(unused)]

use super::object;
use bincode;
use core::time;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fs::{self, File};
use std::io::{BufReader, IoSlice, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;

//use std::thread;

pub fn upload(host: &str, filepath: &str) -> std::io::Result<()> {
    if let Ok(mut stream) = TcpStream::connect(host) {
        let dataset = self::obtain_bytes(filepath)?;
        let filename = String::from(
            PathBuf::from(filepath)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or(filepath),
        );

        let object = object::Object {
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
        //todo: send both the byte data, and filename in one request
    } else {
        println!("🍜 soi | failed to connect to host");
        //if attempts > 3 {
        //    println!("lmao rip");
        //    return Ok(());
        //}
        //thread::sleep(time::Duration::from_secs(1));
        //attempts += 1;
        //upload(host, filename, attempts);
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
