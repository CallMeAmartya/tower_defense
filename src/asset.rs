use macroquad::audio::*;
use macroquad::prelude::*;
use crate::sprite::SpriteSheet;

pub struct Assets {
    pub menu_music: Sound,
    pub combat_music: Sound,
    pub viking_sheet: SpriteSheet,
    pub soldier_sheet: SpriteSheet,
    pub knife_texture: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        let viking_tex = load_texture("assets/sprites/Viking.png")
            .await
            .expect("Failed to load Viking sprite");
        let soldier_tex = load_texture("assets/sprites/German_Soldier.png")
            .await
            .expect("Failed to load German_Soldier sprite");
        let knife_tex = load_texture("assets/sprites/Knife01.png")
            .await
            .expect("Failed to load Knife sprite");
        knife_tex.set_filter(FilterMode::Nearest);

        Self {
            menu_music: load_sound("assets/music/viking_ambient.wav")
                .await
                .expect("Failed to load menu music"),
            combat_music: load_sound("assets/music/combat_fight_loop.wav")
                .await
                .expect("Failed to load combat music"),
            viking_sheet: SpriteSheet::new(viking_tex, 8, 3),
            soldier_sheet: SpriteSheet::new(soldier_tex, 8, 8),
            knife_texture: knife_tex,
        }
    }
}
