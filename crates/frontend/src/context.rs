use std::collections::HashMap;

use backend::game_providers::hoyoplay::proto::{Content, Game};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Context {
    pub api_games: Vec<Game>,
    pub api_news: HashMap<String, Content>,
}
