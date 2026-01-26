use macroquad::audio::*;

pub struct Assets {
    pub menu_music: Sound,
    pub combat_music: Sound,
}

impl Assets {
    pub async fn load() -> Self {

        Self {
            menu_music: load_sound("assets/music/viking_ambient.wav").await.expect("Failed to load menu music"),
            combat_music: load_sound("assets/music/combat_fight_loop.wav").await.expect("Failed to load combat music"),
        }
    }
}