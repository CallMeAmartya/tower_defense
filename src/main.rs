use macroquad::prelude::*;

struct Player {
    pos: Vec2,
    speed: f32,
    health: f32,
    max_health: f32,
    damage: f32,
    projectile_count: u32,
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

    fn upgrade_damage(&mut self, amount: f32) {
        self.damage += amount;
    }

    fn upgrade_health(&mut self, amount: f32) {
        self.max_health += amount;
        self.health += amount;
    }

    fn upgrade_projectiles(&mut self) {
        self.projectile_count *= 2;
    }

    fn upgrade_speed(&mut self, amount: f32) {
        self.speed += amount;
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

enum UpgradeType {
    Damage,
    Projectiles,
    SlowTrap,
    Health,
    Speed,
}

struct UpgradeGate {
    pos: Vec2,
    upgrade_type: UpgradeType,
    health: f32,
    width: f32,
    height: f32,
    lifetime: f32,
}

impl UpgradeGate {
    fn new(pos: Vec2, upgrade_type: UpgradeType, health: f32) -> Self {
        Self {
            pos,
            upgrade_type,
            health,
            width: 60.0,
            height: 80.0,
            lifetime: 15.0,
        }
    }

    fn draw(&self) {
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

    fn check_collision(&self, player_pos: Vec2, player_radius: f32) -> bool {
        // Find the closest point on the rectangle to the circle center
        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;

        let closest_x = player_pos.x.clamp(self.pos.x - half_w, self.pos.x + half_w);
        let closest_y = player_pos.y.clamp(self.pos.y - half_h, self.pos.y + half_h);

        // If distance from circle center to closest point < radius, collision!
        let dist = ((player_pos.x - closest_x).powi(2) + (player_pos.y - closest_y).powi(2)).sqrt();
        dist < player_radius
    }

    fn update(&mut self, dt: f32) {
        self.lifetime -= dt;
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

fn spawn_upgrade_gate() -> UpgradeGate {
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

fn apply_upgrade(upgrade_type: &UpgradeType, player: &mut Player, enemies: &mut Vec<Enemy>) {
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

#[macroquad::main("TowerDefense")]
async fn main() {
    // create player
    let mut player = Player {
        pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        speed: 200.0,
        health: 100.0,
        max_health: 100.0,
        damage: 5.0,
        projectile_count: 1,
        radius: 20.0,
    };

    let mut enemies: Vec<Enemy> = Vec::new();
    let mut projectiles: Vec<Projectile> = Vec::new();
    let mut upgrade_gates: Vec<UpgradeGate> = Vec::new();

    // define projectiles propertiers
    let mut attack_timer = 0.0;
    let attack_cooldown = 0.4;
    let projectile_speed = 400.0;
    let projectile_damage = 0.0;

    // enemy properties
    let mut spawn_timer = 0.0;
    let mut spawn_interval = 2.0;

    // upgrade gate properties
    let mut gate_spawn_timer = 0.0;
    let mut gate_spawn_interval = 5.0;

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

        // spawn upgrade gates
        gate_spawn_timer += dt;
        if gate_spawn_timer >= gate_spawn_interval && upgrade_gates.len() < 3 {
            upgrade_gates.push(spawn_upgrade_gate());
            gate_spawn_timer = 0.0;
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

        // update gates
        for gate in &mut upgrade_gates {
            gate.update(dt);
        }

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

        for gate in &mut upgrade_gates {
            if gate.is_alive() && gate.check_collision(player.pos, player.radius) {
                apply_upgrade(&gate.upgrade_type, &mut player, &mut enemies);
                gate.lifetime = 0.0;
            }
        }

        // remove dead projectiles
        projectiles.retain(|projectile| projectile.is_alive());

        // count enemies before removing dead enemies
        let enemy_count_before = enemies.len();

        // remove dead enemies
        enemies.retain(|enemy| enemy.is_alive());

        // remove gate
        upgrade_gates.retain(|gate| gate.is_alive());

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
