use super::{packet, utils};
use bincode;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task;

pub async fn upload_unix(host: &str, filepath: &str) -> std::io::Result<()> {
    //upload_unix() works under the assumption that both devices share similar endianess:
    //  -macOS uses ARM64  x86_64-based hardware: LITTLE ENDIAN
    //  -Most Linux systems today run on x86, x86_64, and ARM architectures, which are all little-endian by default.
    //
    //note:
    //  -upload_unix() will not overwrite any data if necessary. if the client wishes to not do this,
    //  they must use upload_force_unix().
    //  -this does not work on Windows.

    let filepath_buffer = PathBuf::from(filepath);
    match filepath_buffer.try_exists() {
        Ok(exists) => {
            if !exists {
                println!("🍜 soi | {filepath} does not exist");
                return Ok(());
            }
        }
        Err(error) => {
            println!("🍜 soi | failure checking the path: {:?}", error);
            return Err(error);
        }
    }

    let filename = String::from(
        filepath_buffer
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or(filepath),
    );

    if let Ok(mut stream) = std::net::TcpStream::connect(host) {
        let (tx, mut rx) = mpsc::channel(1);

        let filename_thread = filename.clone();
        let host_thread = host.to_string();

        let _ = task::spawn(async move {
            loop {
                if rx.recv().await.unwrap() == 1 {
                    println!("🍜 soi | shipped {} to {}", filename_thread, host_thread);
                    return;
                }
            }
        });

        println!("🍜 soi | shipping {filename}");

        let dataset = utils::obtain_bytes(filepath)?;
        let packet = packet::Packet {
            command: String::from("upload"),
            filename: filename,
            data: dataset.0,
            size: dataset.1,
        };

        let handle = tokio::spawn(async move {
            if let Ok(packet) = bincode::serialize(&packet) {
                stream
                    .write_all(&packet)
                    .expect("🍜 soi | failed to ship to host");
                tx.send(1).await.unwrap();
            };
        });
        let _ = handle.await;
    } else {
        println!("🍜 soi | failed to connect to host");
    }
    Ok(())
}

pub async fn upload_force_unix(host: &str, filepath: &str) -> std::io::Result<()> {
    //upload_force_unix() works under the assumption that both devices share similar endianess:
    //  -macOS uses ARM64  x86_64-based hardware: LITTLE ENDIAN
    //  -Most Linux systems today run on x86, x86_64, and ARM architectures, which are all little-endian by default.
    //
    //note:
    //  -upload_force_unix() will overwrite any data if necessary. if the client wishes to not do this,
    //  they must use upload_unix().
    //  -this does not work on Windows.

    let filepath_buffer = PathBuf::from(filepath);
    match filepath_buffer.try_exists() {
        Ok(exists) => {
            if !exists {
                println!("🍜 soi | {filepath} does not exist");
                return Ok(());
            }
        }
        Err(error) => {
            println!("🍜 soi | failure checking the path: {:?}", error);
            return Err(error);
        }
    }

    let filename = String::from(
        filepath_buffer
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or(filepath),
    );

    if let Ok(stream) = TcpStream::connect(host).await {
        let (tx, mut rx) = mpsc::channel(1);

        let filename_thread = filename.clone();
        let host_thread = host.to_string();

        let _ = task::spawn(async move {
            loop {
                if rx.recv().await.unwrap() == 1 {
                    println!("🍜 soi | shipped {} to {}", filename_thread, host_thread);
                    return;
                }
            }
        });

        println!("🍜 soi | shipping {filename}");

        let dataset = utils::obtain_bytes(filepath)?;
        let packet = packet::Packet {
            command: String::from("upload--force"),
            filename: filename,
            data: dataset.0,
            size: dataset.1,
        };

        let handle = tokio::task::spawn(async move {
            if let Ok(packet) = bincode::serialize(&packet) {
                stream
                    .try_write(&packet)
                    .expect("🍜 soi | failed to ship to host");
                tx.send(1).await.unwrap();
            }
        });
        let _ = handle.await;
    } else {
        println!("🍜 soi | failed to connect to host");
    }
    Ok(())
}

pub async fn download_unix(host: &str, filepath: &str) -> std::io::Result<()> {
    let filepath_buffer = PathBuf::from(filepath);
    let filename = String::from(filepath_buffer.to_str().unwrap_or(filepath));

    let packet = packet::Packet {
        command: String::from("download"),
        filename: filename.clone(),
        data: vec![0, 1],
        size: 0,
    };
    let packet = bincode::serialize(&packet).unwrap();

    let mut stream = std::net::TcpStream::connect(host)?;
    stream.write_all(&packet).unwrap();

    //TODO: response buffer has to be adaptable based on the file size. in the example of using the girl in space jpg,
    //this works. hwoever, nothign else will work obv due to the fact that the buffer is either too alrge or too small.
    let mut response = [0; 606242];
    stream.read_exact(&mut response).unwrap();

    fs::write(filename, response)?;
    Ok(())
}
