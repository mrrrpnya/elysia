use std::collections::HashMap;

use crate::game_providers::{api::Content, hoyoplay::api::Game};

#[derive(Debug, Clone)]
pub struct Context {
    pub api_games: Vec<Game>,
    pub api_news: HashMap<String, Content>,
}
