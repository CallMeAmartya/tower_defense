use macroquad::prelude::*;
use crate::sprite::SpriteSheet;

pub struct Player {
    pub pos: Vec2,
    pub speed: f32,
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
    pub projectile_count: u32,
    pub projectile_speed: f32,
    pub radius: f32,
    pub anim_frame: usize,
    pub anim_timer: f32,
    pub facing_left: bool,
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
            anim_frame: 0,
            anim_timer: 0.0,
            facing_left: false,
        }
    }
    // update player
    pub fn update(&mut self, dt: f32) {
        let mut direction = Vec2::ZERO;

        // Keyboard input
        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
        }
        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
        }

        // Touch input (for mobile)
        for touch in touches() {
            if touch.phase == TouchPhase::Started || touch.phase == TouchPhase::Moved {
                let touch_dir = touch.position - self.pos;
                if touch_dir.length() > 30.0 {  // Dead zone
                    direction = touch_dir.normalize();
                }
            }
        }

        // Apply movement
        if direction != Vec2::ZERO {
            direction = direction.normalize();
            self.pos += direction * self.speed * dt;

            // Track facing direction
            if direction.x < 0.0 {
                self.facing_left = true;
            } else if direction.x > 0.0 {
                self.facing_left = false;
            }

            // Advance animation while moving
            self.anim_timer += dt;
            if self.anim_timer >= 0.12 {
                self.anim_timer = 0.0;
                self.anim_frame = (self.anim_frame + 1) % 8;
            }
        } else {
            // Reset to idle frame when standing still
            self.anim_frame = 0;
            self.anim_timer = 0.0;
        }

        // Clamp to screen
        self.pos.x = self.pos.x.clamp(self.radius, screen_width() - self.radius);
        self.pos.y = self.pos.y.clamp(self.radius, screen_height() - self.radius);
    }

    // draw player using sprite sheet
    pub fn draw(&self, sheet: &SpriteSheet) {
        let dest_height = self.radius * 2.5;
        sheet.draw_frame(self.anim_frame, 0, self.pos.x, self.pos.y, dest_height, self.facing_left);
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
