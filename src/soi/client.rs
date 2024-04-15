use super::{packet, utils};
use bincode;
use core::time;
use std::io::Write;
use std::net::TcpStream;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

const UPLOAD_RETRY_COUNT: u8 = 3;

pub fn upload_force_unix(host: &str, filepath: &str, mut attempts: u8) -> std::io::Result<()> {
    //upload_force_unix() works under the assumption that both devices share similar endianess:
    //  -macOS uses ARM64  x86_64-based hardware: LITTLE ENDIAN
    //  -Most Linux systems today run on x86, x86_64, and ARM architectures, which are all little-endian by default.
    //
    //additionally:
    //  -upload_force_unix() will overwrite any data if necessary. if the client wishes to not do this,
    //  they will use upload_unix().

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

    if let Ok(mut stream) = TcpStream::connect(host) {
        let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();
        let filename_thread = filename.clone();
        let host_thread = host.to_string();

        let _ = std::thread::spawn(move || loop {
            if rx.recv().unwrap() == 1 {
                println!("🍜 soi | shipped {} to {}", filename_thread, host_thread);
                return;
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

        if let Ok(packet) = bincode::serialize(&packet) {
            stream
                .write(&packet)
                .expect("🍜 soi | failed to ship to host");
            tx.send(1).unwrap();
        };
    } else {
        println!("🍜 soi | failed to connect to host, trying again in 3 seconds...");
        if attempts >= UPLOAD_RETRY_COUNT {
            println!("🍜 soi | lmao rip");
            return Ok(());
        }
        std::thread::sleep(time::Duration::from_secs(3));
        attempts += 1;
        upload_force_unix(host, filepath, attempts).unwrap();
    }
    Ok(())
}

pub fn download_unix(host: &str, filepath: &str) -> std::io::Result<()> {
    if let Ok(mut stream) = TcpStream::connect(host) {
        let filepath_buffer = PathBuf::from(filepath);
        let filename = String::from(filepath_buffer.to_str().unwrap_or(filepath));

        let cmd = String::from("download");
        let packet = packet::Packet {
            command: cmd,
            filename: filename,
            data: Vec::new(),
            size: 0,
        };

        if let Ok(packet) = bincode::serialize(&packet) {
            stream
                .write(&packet)
                .expect("🍜 soi | failed to download from host");
            println!("🍜 soi | request for {filepath} sent to {host}");
        };

        return Ok(()); //ends the stream

        //note to self:
        //so heres the shit- the server waits for the stream to be complete in order to
        //process the issue. what i mean by complete, is that this function needs to return.
        //this is obviously a major fucking issue!
        //because, althought we successfully sent the packet request for download, the server wont
        //actually write it into the stream UNLESS this function/client is returned.
        //
        //so how the fuck do we fix this? its clearly a bug of shitty code im ngl.
        //
        //in orcder for us to read the bytes, keeping the stream alive, we gotta refactor this
        //whole shit (probably)
        //
        //let bytes = [0; 10];
        //stream.read(&mut bytes).expect("fuck");
        //println!("{:?}", bytes);
        //
        //the server isnt sending anything while this stream is active.
        //
        //maybe we need some thread shit or soemthing, tokio perhaps?
    }
    Ok(())
}
