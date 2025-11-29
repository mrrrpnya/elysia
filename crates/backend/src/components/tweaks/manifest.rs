use std::collections::HashMap;

pub struct TweakManifest {
    game_tweaks: HashMap<String, Vec<String>>,
}

impl TweakManifest {
    pub fn new() -> Self {
        let mut game_tweaks = HashMap::new();

        game_tweaks.insert(
            // Endfield
            "endfield_zePXHT2t4L2tKR4m".to_string(),
            vec!["jade".to_string()],
        );
        
        Self { game_tweaks }
    }
    
    pub fn needs_jade(&self, game_id: &str) -> bool {
        self.game_tweaks
            .get(game_id)
            .map(|tweaks| tweaks.contains(&"jade".to_string()))
            .unwrap_or(false)
    }
    
    pub fn get_tweaks(&self, game_id: &str) -> Vec<String> {
        self.game_tweaks
            .get(game_id)
            .cloned()
            .unwrap_or_default()
    }
}