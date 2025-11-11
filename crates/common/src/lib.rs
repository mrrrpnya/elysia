use std::sync::LazyLock;

use reqwest::Client;

pub mod github;
pub mod utils;

pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("Elysia/1.0.0")
        .build()
        .unwrap_or_else(|_| Client::new())
});
