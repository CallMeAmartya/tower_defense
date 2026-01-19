use macroquad::prelude::*;

pub struct Enemy {
    pub pos: Vec2,
    pub speed: f32,
    pub health: f32,
    pub radius: f32,
}

impl Enemy {
    // update enemy
    pub fn update(&mut self, player_pos: Vec2, dt: f32) {
        let direction = (player_pos - self.pos).normalize_or_zero();
        self.pos += direction * self.speed * dt;
    }

    // draw enemy
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, RED);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }
}

pub fn find_closest_enemy(player_pos: Vec2, enemies: &[Enemy]) -> Option<Vec2> {
    if enemies.is_empty() {
        return None;
    }

    let mut closest_pos = enemies[0].pos;
    let mut closest_dist = (enemies[0].pos - player_pos).length();

    for enemy in enemies {
        let dist = (enemy.pos - player_pos).length();
        if dist < closest_dist {
            closest_dist = dist;
            closest_pos = enemy.pos;
        }
    }

    Some(closest_pos)
}

pub fn spawn_enemy(wave: u32) -> Enemy {
    let side: i32 = rand::gen_range(0, 4);

    let pos: Vec2 = match side {
        0 => Vec2::new(rand::gen_range(0.0, screen_width()), -30.0), // top
        1 => Vec2::new(rand::gen_range(0.0, screen_width()), screen_height() + 30.0), // bottom
        2 => Vec2::new(-30.0, rand::gen_range(0.0, screen_height())), // left
        _ => Vec2::new(screen_width() + 30.0, rand::gen_range(0.0, screen_height())), // right
    };

    let speed: f32 = rand::gen_range(100.0, 150.0);

    let radius: f32 = 60.0 - (0.3 * speed);

    // Scale stats with wave
    let wave_multiplier = 1.0 + (wave as f32 - 1.0) * 1.0; // +100% per wave

    Enemy {
        pos,
        speed: rand::gen_range(50.0, 80.0) * wave_multiplier,
        health: 50.0 * wave_multiplier,
        radius: radius,
    }
}
