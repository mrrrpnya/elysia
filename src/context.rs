use std::collections::HashMap;

use crate::game_providers::hoyoplay::api::{Content, Game};

#[derive(Debug, Clone)]
pub struct Context {
    pub api_games: Vec<Game>,
    pub api_news: HashMap<String, Content>,
}
