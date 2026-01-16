use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub speed: f32,
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
    pub projectile_count: u32,
    pub projectile_speed: f32,
    pub radius: f32,
}

impl Player {
    // create player
    pub fn new() -> Self {
        Self {
            pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            speed: 200.0,
            health: 100.0,
            max_health: 100.0,
            damage: 5.0,
            projectile_count: 1,
            projectile_speed: 200.0,
            radius: 20.0,
        }
    }
    // update player
    pub fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::A) {
            self.pos.x -= self.speed * dt;
        }
        if is_key_down(KeyCode::D) {
            self.pos.x += self.speed * dt;
        }
        if is_key_down(KeyCode::W) {
            self.pos.y -= self.speed * dt;
        }
        if is_key_down(KeyCode::S) {
            self.pos.y += self.speed * dt;
        }
        self.pos.x = self.pos.x.clamp(self.radius, screen_width() - self.radius);
        self.pos.y = self.pos.y.clamp(self.radius, screen_height() - self.radius);
    }

    // draw player
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, GREEN);
    }

    pub fn upgrade_damage(&mut self, amount: f32) {
        self.damage += amount;
    }

    pub fn upgrade_health(&mut self, amount: f32) {
        self.max_health += amount;
        self.health += amount;
    }

    pub fn upgrade_projectiles(&mut self) {
        self.projectile_count *= 2;
    }

    pub fn upgrade_speed(&mut self, amount: f32) {
        self.speed += amount;
    }
}
