use support::env_param;
use tokio::{
    signal::unix::{signal, SignalKind},
    task,
};
use transport::{discord::DiscordTransport, telegram::TelegramTransport};

#[derive(Clone)]
struct App;

impl support::traits::Config for App {}

env_param!(Name, "EDDIE_NAME", "Eddie");
env_param!(WalletSeed, "EDDIE_WALLET_SEED", "\\ALICE");
env_param!(SubstrateRPC, "EDDIE_SUBSTRATE_RPC", "ws://127.0.0.1:9944");
impl eddie_lib::Config for App {
    type Name = Name;
    type WalletSeed = WalletSeed;
    type SubstrateRPC = SubstrateRPC;
}

env_param!(DiscordToken, "EDDIE_DISCORD_TOKEN");
impl transport::discord::Config for App {
    type Bot = Self;
    type Token = DiscordToken;
}

env_param!(TelegramToken, "EDDIE_TELEGRAM_TOKEN");
impl transport::telegram::Config for App {
    type Bot = Self;
    type Token = TelegramToken;
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

    let telegram_task = task::spawn(async move {
        let telegram = TelegramTransport::<App>::new();
        telegram.serve().await;
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
        _ = telegram_task => {}
    }
}
