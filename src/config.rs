use macroquad::prelude::*;

pub struct Theme {
    pub game_title: &'static str,
    pub player_color: Color,
    pub enemy_color: Color,

    pub damage_upgrade_name: &'static str,
    pub projectile_upgrade_name: &'static str,
    pub slow_upgrade_name: &'static str,
    pub health_upgrade_name: &'static str,
    pub speed_upgrade_name: &'static str,
}

pub const VIKING_RUSH_THEME: Theme = Theme {
    game_title: "Viking Rush",
    player_color: Color::new(0.0, 0.66, 0.42, 1.0),
    enemy_color: Color::new(0.24, 0.23, 0.43, 1.0),

    damage_upgrade_name: "Hakarl Buff",
    projectile_upgrade_name: "Viking heritage",
    slow_upgrade_name: "Permafrost Trap",
    health_upgrade_name: "Arctic Resilience",
    speed_upgrade_name: "Sledge Dogs",
};
