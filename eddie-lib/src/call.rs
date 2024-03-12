// #[derive(Clone, Debug, PartialEq)]
// Collection of all possible origins.
// pub enum Origin {
//     Telegram,
//     Discord,
// }

#[derive(Clone, Debug, PartialEq)]
/// Collection of all possible calls to the bot.
pub enum Call {
    Version,
}

#[derive(Clone, Debug, PartialEq)]
/// Collection of all possible responses from the bot.
pub enum Response {
    Version(String),
}
