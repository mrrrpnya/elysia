use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<DataType> {
    pub retcode: i32,
    pub message: String,
    pub data: DataType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGames {
    pub games: Vec<Game>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGameContent {
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub biz: String,
    pub display: Display,
    pub reservation: Option<serde_json::Value>, // null or future object
    pub display_status: String,
    #[serde(default)]
    pub game_server_configs: Vec<GameServerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Display {
    pub language: String,
    pub name: String,
    pub icon: Image,
    pub title: String,
    pub subtitle: String,
    pub background: ImageLink,
    pub logo: ImageLink,
    pub thumbnail: ImageLink,
    pub korea_rating: Option<serde_json::Value>,
    pub shortcut: Image,
    pub wpf_icon: Option<Image>, // sometimes null
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub hover_url: String,
    pub link: String,
    pub login_state_in_link: bool,
    pub md5: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLink {
    pub url: String,
    pub link: String,
    pub login_state_in_link: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameServerConfig {
    pub i18n_name: String,
    pub i18n_description: String,
    pub package_name: String,
    pub auto_scan_registry_key: String,
    pub package_detection_info: String,
    pub game_id: String,
    pub reservation: Option<serde_json::Value>,
    pub display_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub game: GameInfo,
    pub language: String,
    pub banners: Vec<Banner>,
    pub posts: Vec<Post>,
    pub social_media_list: Vec<SocialMedia>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub id: String,
    pub biz: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Banner {
    pub id: String,
    pub image: ImageLink,
    pub i18n_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    #[serde(rename = "type")]
    pub post_type: String,
    pub title: String,
    pub link: String,
    pub date: String,
    pub login_state_in_link: bool,
    pub i18n_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialMedia {
    pub id: String,
    pub icon: Image,
    pub qr_image: ImageLink,
    pub qr_desc: String,
    #[serde(default)]
    pub links: Vec<serde_json::Value>, // empty array in sample
    pub enable_red_dot: bool,
    pub red_dot_content: String,
}
