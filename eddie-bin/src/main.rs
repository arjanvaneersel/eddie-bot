use support::env_param;
use tokio::{
    signal::unix::{signal, SignalKind},
    task,
};
use transport::discord::DiscordTransport;

#[derive(Clone)]
struct App;

impl support::traits::Config for App {}

env_param!(Name, "EDDIE_NAME");
impl eddie_lib::Config for App {
    type Name = Name;
    type WalletSeed = ();
    type SubstrateRPC = ();
}

env_param!(DiscordToken, "EDDIE_DISCORD_TOKEN");
impl transport::discord::Config for App {
    type Bot = Self;
    type Token = DiscordToken;
}

#[tokio::main]
async fn main() {
    // let telegram_token = env::var("EDDIE_TELEGRAM_TOKEN").expect("No EDDIE_TELEGRAM_TOKEN in env");
    pretty_env_logger::init();

    let discord_task = task::spawn(async move {
        let discord = DiscordTransport::<App>::new();
        if let Err(err) = discord.serve().await {
            log::error!("{}", err)
        }
    });

    // Handle termination signal (CTRL+C)
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = async {
            sigint.recv().await;
            println!("Received SIGINT. Shutting down gracefully...");
        } => {}
        _ = async {
            sigterm.recv().await;
            println!("Received SIGTERM. Shutting down gracefully...");
        } => {}
        _ = discord_task => {}
        // _ = telegram_task => {}
    }
}
