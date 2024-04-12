mod soi;

fn main() -> std::io::Result<()> {
    let cmd = std::env::args().nth(1).expect("no pattern given");

    match cmd.as_str() {
        "launch" => {
            if let Ok(mut soi_instance) = soi::server::build() {
                soi_instance.launch().expect("🍜 soi | failed to launch");
            }
        }
        "upload" => soi::client::upload("127.0.0.1:8080", "Makefile")?,
        &_ => todo!(),
    }

    Ok(())
}
