use crate::enemy::Enemy;
use crate::player::Player;
use macroquad::prelude::*;
pub struct Projectile {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub damage: f32,
    pub radius: f32,
    pub lifetime: f32,
}

impl Projectile {
    pub fn update(&mut self, dt: f32) {
        self.pos += self.velocity * dt;
        self.lifetime -= dt;
    }

    pub fn draw(&self) {
        draw_circle(
            self.pos.x,
            self.pos.y,
            self.radius * 1.5,
            Color::from_rgba(255, 255, 100, 100),
        );
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }

    pub fn hits(&self, enemy: &Enemy) -> bool {
        let distance = (self.pos - enemy.pos).length();
        distance < self.radius + enemy.radius
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

pub fn fire_projectiles(player: &Player, target_pos: Vec2, projectiles: &mut Vec<Projectile>) {
    // calculate direction to target
    let base_direction = (target_pos - player.pos).normalize_or_zero();

    // spread angle between projectiles
    let spread = 0.2;
    let total_spread = spread * (player.projectile_count - 1) as f32;
    let start_angle = -total_spread / 2.0;

    for i in 0..player.projectile_count {
        let angle = start_angle + spread * i as f32;

        // rotate direnction by angle
        let direction = Vec2::new(
            base_direction.x * angle.cos() - base_direction.y * angle.sin(),
            base_direction.x * angle.sin() + base_direction.y * angle.cos(),
        );
        // create projectile
        projectiles.push(Projectile {
            pos: player.pos,
            velocity: direction * player.projectile_speed,
            damage: player.damage,
            radius: 6.0,
            lifetime: 2.0,
        });
    }
}
