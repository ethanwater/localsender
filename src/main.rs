mod soi;

fn main() -> std::io::Result<()> {
    let cmd = std::env::args().nth(1).expect("no pattern given");
    let arg = std::env::args().nth(2).unwrap_or(String::from(""));
    let os = std::env::consts::OS;

    match cmd.as_str() {
        "launch" => {
            if let Ok(mut soi_instance) = soi::server::build() {
                soi_instance.launch().expect("🍜 soi | failed to launch");
            }
        }
        "upload" => match os {
            "macos" | "linux" => soi::client::upload_unix("127.0.0.1:8080", arg.as_str(), 0)?,
            "windows" => todo!(),
            &_ => todo!(),
        },
        "download" => match os {
            "macos" | "linux" => soi::client::download("127.0.0.1:8080", arg.as_str())?,
            "windows" => todo!(),
            &_ => todo!(),
        },

        &_ => todo!(),
    }

    Ok(())
}
