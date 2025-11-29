use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProxyResponse {
    pub proxy_rsps: Vec<ProxyRsp>,
    pub timestamp: Option<String>,
    pub seq: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyRsp {
    pub kind: String,
    #[serde(rename = "get_latest_game_rsp")]
    pub get_latest_game_rsp: Option<GetLatestGameRsp>,
    #[serde(rename = "get_main_bg_image_rsp")]
    pub get_main_bg_image_rsp: Option<GetMainBgImageRsp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLatestGameRsp {
    pub action: Option<i32>,
    pub version: Option<String>,
    pub request_version: Option<String>,
    pub pkg: Option<Pkg>,
    pub patch: Option<serde_json::Value>,
    pub state: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pkg {
    pub packs: Vec<Pack>,
    pub total_size: Option<String>,
    pub file_path: Option<String>,
    pub url: Option<String>,
    pub md5: Option<String>,
    pub package_size: Option<String>,
    pub file_id: Option<String>,
    pub sub_channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pack {
    pub url: String,
    pub md5: Option<String>,
    pub package_size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMainBgImageRsp {
    pub main_bg_image: Option<MainBgImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainBgImage {
    pub url: String,
    pub md5: Option<String>,
    pub size: Option<i32>,
}
