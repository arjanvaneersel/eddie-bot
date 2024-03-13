use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
/// The caller origin.
pub enum Origin {
    Discord(String),
    Telegram(String),
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:?}", self).replace("\"", "");
        write!(f, "{}", s)
    }
}

impl support::traits::Origin for Origin {
    fn network(&self) -> String {
        match self {
            Self::Discord(_) => "DISCORD".to_string(),
            Self::Telegram(_) => "TELEGRAM".to_string(),
        }
    }

    fn user_id(&self) -> String {
        match self {
            Self::Discord(user) => user.into(),
            Self::Telegram(user) => user.into(),
        }
    }
}

fn parse_origin(input: &str) -> Option<Origin> {
    let discord_regex = Regex::new(r#"Discord\((\w+)\)"#).unwrap();
    let telegram_regex = Regex::new(r#"Telegram\((\w+)\)"#).unwrap();

    if let Some(captures) = discord_regex.captures(input) {
        let id = captures.get(1).unwrap().as_str().to_string();
        return Some(Origin::Discord(id));
    } else if let Some(captures) = telegram_regex.captures(input) {
        let id = captures.get(1).unwrap().as_str().to_string();
        return Some(Origin::Telegram(id));
    }

    None
}

impl TryFrom<&str> for Origin {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match parse_origin(value) {
            Some(val) => Ok(val),
            None => Err("Invalid origin"),
        }
    }
}

impl TryFrom<String> for Origin {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match parse_origin(&value) {
            Some(val) => Ok(val),
            None => Err("Invalid origin"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_to_string_and_debug_works() {
        let discord_origin = Origin::Discord("1234".into());
        let telegram_origin = Origin::Telegram("4321".into());
        assert_eq!(discord_origin.to_string(), "Discord(1234)");
        assert_eq!(telegram_origin.to_string(), "Telegram(4321)");

        assert_eq!(format!("{}", discord_origin), "Discord(1234)");
        assert_eq!(format!("{}", telegram_origin), "Telegram(4321)");

        assert_eq!(format!("{:?}", discord_origin), "Discord(\"1234\")");
        assert_eq!(format!("{:?}", telegram_origin), "Telegram(\"4321\")");
    }

    #[test]
    fn try_string_to_origin_works() {
        assert_eq!(
            Origin::try_from("Discord(1234)").unwrap(),
            Origin::Discord("1234".into())
        );
        assert_eq!(
            Origin::try_from("Telegram(4321)").unwrap(),
            Origin::Telegram("4321".into())
        );

        assert!(Origin::try_from("Blah(4321)").is_err());
    }
}
