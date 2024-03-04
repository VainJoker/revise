use serde::Deserialize;
use crate::error::ReviseResult;

#[derive(Deserialize)]
pub struct Config{
    pub messages: Message,
    pub emoji: Emoji,
    pub emoji_align: String,
    pub scopes: Vec<String>,
}

#[derive(Deserialize)]
pub struct Message{

}

#[derive(Deserialize)]
pub struct Emoji{

}


impl Default for Config {
    fn default() -> Self {
        todo!()
    }
}

pub fn toml_parser() -> ReviseResult<Config>{
    Ok(Config::default())
}