use lyceris::minecraft::{
    config::ConfigBuilder,
    emitter::{Emitter, Event},
    install::install,
    launch::launch,
};
use lyceris::minecraft::loader::neoforge::NeoForge;

use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let emitter = Emitter::default();

    emitter
        .on(
            Event::SingleDownloadProgress,
            |(path, current, total): (String, u64, u64)| {
                println!("Downloading {} - {}/{}", path, current, total);
            },
        )
        .await;

    emitter
        .on(Event::Console, |line: String| {
            println!("Line: {}", line);
        })
        .await;

    let local_appdata = std::env::var("LOCALAPPDATA")?;


    let config = ConfigBuilder::new(
        PathBuf::from(local_appdata).join("OliviaLauncher"),
        "1.21.1".into(),
        lyceris::auth::AuthMethod::Offline {
            username: "Lyceris".into(),
            uuid: None,

        },
    )
        .loader(NeoForge("21.1.233".to_string()).into())
        .build();

    install(&config, Some(&emitter)).await?;

    let mut child = launch(&config, Some(&emitter)).await?;
    child.wait().await?;

    Ok(())
}

//test 1