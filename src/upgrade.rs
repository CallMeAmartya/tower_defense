use crate::enemy::Enemy;
use crate::player::Player;
use macroquad::prelude::*;

pub enum UpgradeType {
    Damage,
    Projectiles,
    SlowTrap,
    Health,
    Speed,
}
pub struct UpgradeGate {
    pub pos: Vec2,
    pub upgrade_type: UpgradeType,
    pub health: f32,
    pub width: f32,
    pub height: f32,
    pub lifetime: f32,
}

impl UpgradeGate {
    pub fn new(pos: Vec2, upgrade_type: UpgradeType, health: f32) -> Self {
        Self {
            pos,
            upgrade_type,
            health,
            width: 60.0,
            height: 80.0,
            lifetime: 4.0,
        }
    }

    pub fn draw(&self) {
        // different colors based on upgrade type
        let color = match self.upgrade_type {
            UpgradeType::Damage => Color::from_rgba(255, 100, 50, 255),
            UpgradeType::Projectiles => Color::from_rgba(100, 200, 255, 255),
            UpgradeType::SlowTrap => Color::from_rgba(150, 200, 255, 255),
            UpgradeType::Health => Color::from_rgba(100, 255, 100, 255),
            UpgradeType::Speed => Color::from_rgba(255, 255, 100, 255),
        };

        // label
        let label = match self.upgrade_type {
            UpgradeType::Damage => "DMG",
            UpgradeType::Projectiles => "x2",
            UpgradeType::SlowTrap => "ICE",
            UpgradeType::Health => "+HP",
            UpgradeType::Speed => "SPD",
        };

        let x = self.pos.x - self.width / 2.0;
        let y = self.pos.y - self.height / 2.0;

        // gate backgroud
        draw_rectangle(
            x,
            y,
            self.width,
            self.height,
            Color::from_rgba(30, 30, 40, 200),
        );

        // gate border
        draw_rectangle_lines(x, y, self.width, self.height, 3.0, color);

        // pillars
        draw_rectangle(x, y, 6.0, self.height, color);
        draw_rectangle(x + self.width - 6.0, y, 6.0, self.height, color);

        // Top bar
        draw_rectangle(x, y, self.width, 8.0, color);

        // label
        let text_size = 20.0;
        let text_width = measure_text(label, None, text_size as u16, 1.0).width;
        draw_text(
            label,
            self.pos.x - text_width / 2.0,
            self.pos.y + 6.0,
            text_size,
            WHITE,
        );

        // Pulsing when about to expire
        if self.lifetime < 3.0 {
            let pulse = (self.lifetime * 10.0).sin().abs();
            draw_rectangle(
                x,
                y,
                self.width,
                self.height,
                Color::from_rgba(255, 255, 255, (pulse * 50.0) as u8),
            );
        }
    }

    pub fn check_collision(&self, player_pos: Vec2, player_radius: f32) -> bool {
        // Find the closest point on the rectangle to the circle center
        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;

        let closest_x = player_pos.x.clamp(self.pos.x - half_w, self.pos.x + half_w);
        let closest_y = player_pos.y.clamp(self.pos.y - half_h, self.pos.y + half_h);

        // If distance from circle center to closest point < radius, collision!
        let dist = ((player_pos.x - closest_x).powi(2) + (player_pos.y - closest_y).powi(2)).sqrt();
        dist < player_radius
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetime -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

pub fn spawn_upgrade_gate() -> UpgradeGate {
    let padding = 80.0;
    let pos = Vec2::new(
        rand::gen_range(padding, screen_width() - padding),
        rand::gen_range(padding, screen_height() - padding),
    );

    // random upgrade type
    let roll = rand::gen_range(0.0, 1.0);
    let (upgrade_type, gate_health) = if roll < 0.25 {
        (UpgradeType::Damage, 10.0)
    } else if roll < 0.50 {
        (UpgradeType::Projectiles, 15.0)
    } else if roll < 0.65 {
        (UpgradeType::SlowTrap, 20.0)
    } else if roll < 0.82 {
        (UpgradeType::Health, 25.0)
    } else {
        (UpgradeType::Speed, 30.0)
    };

    UpgradeGate::new(pos, upgrade_type, gate_health)
}

pub fn apply_upgrade(upgrade_type: &UpgradeType, player: &mut Player, enemies: &mut Vec<Enemy>) {
    match upgrade_type {
        UpgradeType::Damage => {
            player.upgrade_damage(10.0);
        }
        UpgradeType::Projectiles => {
            player.upgrade_projectiles();
        }
        UpgradeType::SlowTrap => {
            for enemy in enemies {
                enemy.speed *= 0.5;
            }
        }
        UpgradeType::Health => {
            player.upgrade_health(25.0);
        }
        UpgradeType::Speed => {
            player.upgrade_speed(30.0);
        }
    }
}
