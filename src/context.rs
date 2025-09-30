use std::{collections::HashMap, path::PathBuf};

use crate::game_providers::hoyoplay::api::{Content, Game};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Context {
    pub api_games: Vec<Game>,
    pub api_news: HashMap<String, Content>,
    pub data_path: PathBuf,
}
