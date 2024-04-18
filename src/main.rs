mod soi;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let os = std::env::consts::OS;
    let (cmd, second, third, fourth) = (
        std::env::args().nth(1).expect("🍜 soi | no command given"),
        std::env::args().nth(2).unwrap_or(String::from("")),
        std::env::args().nth(3).unwrap_or(String::from("")),
        std::env::args().nth(4).unwrap_or(String::from("")),
    );

    //yes, i know this parsing is shit. and yes, i know i should use clap. but respectfully i dont care too atm. rn, im focused on the code.
    //not the fucking parsing!

    match cmd.as_str() {
        "launch" => {
            if let Ok(mut soi_instance) = soi::server::build().await {
                soi_instance
                    .launch()
                    .await
                    .expect("🍜 soi | failed to launch");
            }
        }
        "upload" => match fourth.as_str() {
            "force" => match os {
                "macos" | "linux" => {
                    soi::client::upload_force_unix(second.as_str(), third.as_str()).await?
                }
                "windows" => todo!(),
                &_ => todo!(),
            },
            "" => match os {
                "macos" | "linux" => {
                    soi::client::upload_unix(second.as_str(), third.as_str()).await?
                }
                "windows" => todo!(),
                &_ => todo!(),
            },
            &_ => todo!(),
        },
        "download" => match os {
            "macos" | "linux" => {
                soi::client::download_unix(second.as_str(), third.as_str()).await?
            }
            "windows" => todo!(),
            &_ => todo!(),
        },
        "storage" => match second.as_str() {
            "set" => soi::config::set_storage(&third)?,
            &_ => println!("🍜 soi | invalid command: {second}"),
        },
        &_ => println!("🍜 soi | invalid command: {cmd}"),
    }

    Ok(())
}
