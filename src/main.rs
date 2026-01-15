use macroquad::prelude::*;

struct Player {
    pos: Vec2,
    speed: f32,
    health: f32,
    radius: f32,
}

impl Player {
    // update player
    fn update(&mut self, dt: f32) {
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
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, GREEN);
    }
}

struct Enemy {
    pos: Vec2,
    speed: f32,
    health: f32,
    radius: f32,
}

impl Enemy {
    // update enemy
    fn update(&mut self, player_pos: Vec2, dt: f32) {
        let direction = (player_pos - self.pos).normalize_or_zero();
        self.pos += direction * self.speed * dt;
    }

    // draw enemy
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, RED);
    }

    fn is_alive(&self) -> bool {
        self.health > 0.0
    }
}

struct Projectile {
    pos: Vec2,
    velocity: Vec2,
    damage: f32,
    radius: f32,
    lifetime: f32,
}

impl Projectile {
    fn update(&mut self, dt: f32) {
        self.pos += self.velocity * dt;
        self.lifetime -= dt;
    }

    fn draw(&self) {
        draw_circle(
            self.pos.x,
            self.pos.y,
            self.radius * 1.5,
            Color::from_rgba(255, 255, 100, 100),
        );
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }

    fn hits(&self, enemy: &Enemy) -> bool {
        let distance = (self.pos - enemy.pos).length();
        distance < self.radius + enemy.radius
    }

    fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

fn find_closest_enemy(player_pos: Vec2, enemies: &[Enemy]) -> Option<Vec2> {
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

fn spawn_enemy() -> Enemy {
    let side: i32 = rand::gen_range(0, 4);

    let pos: Vec2 = match side {
        0 => Vec2::new(rand::gen_range(0.0, screen_width()), -30.0), // top
        1 => Vec2::new(rand::gen_range(0.0, screen_width()), screen_height() + 30.0), // bottom
        2 => Vec2::new(-30.0, rand::gen_range(0.0, screen_height())), // left
        _ => Vec2::new(screen_width() + 30.0, rand::gen_range(0.0, screen_height())), // right
    };

    let speed: f32 = rand::gen_range(50.0, 150.0);

    let radius: f32 = 40.0 - (0.2 * speed);

    Enemy {
        pos,
        speed: rand::gen_range(50.0, 80.0),
        health: 30.0,
        radius: radius,
    }
}

#[macroquad::main("TowerDefense")]
async fn main() {
    // create player
    let mut player = Player {
        pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        speed: 200.0,
        health: 100.0,
        radius: 20.0,
    };

    let mut enemies: Vec<Enemy> = Vec::new();
    let mut projectiles: Vec<Projectile> = Vec::new();

    // define projectiles propertiers
    let mut attack_timer = 0.0;
    let attack_cooldown = 0.4;
    let projectile_speed = 400.0;
    let projectile_damage = 0.0;

    // enemy properties
    let mut spawn_timer = 0.0;
    let mut spawn_interval = 2.0;

    // score
    let mut score: u32 = 0;

    loop {
        let dt = get_frame_time();

        // spawn enemies
        spawn_timer += dt;
        if enemies.len() < 20 && spawn_timer >= spawn_interval {
            enemies.push(spawn_enemy());
            spawn_timer = 0.0;
            spawn_interval -= 0.005;
        }

        // auto attack
        attack_timer += dt;
        if attack_timer >= attack_cooldown {
            // find target
            if let Some(target_pos) = find_closest_enemy(player.pos, &enemies) {
                // calculate direction to target
                let direction = (target_pos - player.pos).normalize_or_zero();
                // create projectile
                projectiles.push(Projectile {
                    pos: player.pos,
                    velocity: direction * projectile_speed,
                    damage: projectile_damage,
                    radius: 6.0,
                    lifetime: 2.0,
                });
                attack_timer = 0.0;
            }
        }

        // update enemy pos
        for enemy in &mut enemies {
            enemy.update(player.pos, dt);
        }

        // update player pos
        player.update(dt);

        // update projectile
        for projectile in &mut projectiles {
            projectile.update(dt);
        }

        // collision
        for proj in &mut projectiles {
            for enemy in &mut enemies {
                if proj.hits(enemy) {
                    enemy.health -= proj.damage;
                    proj.lifetime = 0.0; // Mark projectile for removal
                    break; // This projectile can only hit one enemy
                }
            }
        }

        for enemy in &enemies {
            let dist = (enemy.pos - player.pos).length();
            if dist < enemy.radius + player.radius {
                player.health -= 20.0 * dt;
            }
        }

        // remove dead projectiles
        projectiles.retain(|projectile| projectile.is_alive());

        // count enemies before removing dead enemies
        let enemy_count_before = enemies.len();

        // remove dead enemies
        enemies.retain(|enemy| enemy.is_alive());

        // update score
        let kills = enemy_count_before - enemies.len();
        score += kills as u32 * 10; // 10 points per kill

        // check game over
        if player.health <= 0.0 {
            player.health = 100.0;
            player.pos = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
            enemies.clear();
            score = 0;
        }

        // draw frame, player, enemies and projectiles
        clear_background(Color::from_rgba(20, 20, 30, 225));
        player.draw();
        for enemy in &enemies {
            enemy.draw();
        }
        for projectile in &projectiles {
            projectile.draw();
        }

        // HUD
        // draw_text("Tower Defense", 20.0, 40.0, 40.0, WHITE);
        // draw_text("Press SPACE to start", 20.0, 80.0, 20.0, GRAY);
        draw_text(
            &format!("Health: {}", player.health),
            screen_width() - 120.0,
            screen_height() - 40.0,
            24.0,
            WHITE,
        );
        draw_text(
            &format!("Pos: ({:.0}, {:.0})", player.pos.x, player.pos.y),
            screen_width() - 120.0,
            screen_height() - 20.0,
            16.0,
            GRAY,
        );
        draw_text(&format!("Score: {}", score), 20.0, 30.0, 24.0, WHITE);

        // wait for next frame
        next_frame().await
    }
}
