mod config;
mod enemy;
mod player;
mod projectile;
mod ui;
mod upgrade;
mod asset;

use config::VIKING_RUSH_THEME;
use enemy::Enemy;
use enemy::find_closest_enemy;
use enemy::spawn_enemy;
use macroquad::audio::PlaySoundParams;
use macroquad::audio::play_sound;
use macroquad::audio::stop_sound;
use macroquad::prelude::*;
use player::Player;
use projectile::Projectile;
use projectile::fire_projectiles;
use ui::draw_game_over;
use ui::draw_hud;
use ui::draw_menu;
use upgrade::UpgradeGate;
use upgrade::apply_upgrade;
use upgrade::spawn_upgrade_gate;
use asset::Assets;

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
    GameOver,
}

fn start_wave(wave: u32) -> u32 {
    // return number of enemeies to spawn
    10 + (wave.pow(2))
}

#[macroquad::main("VikingRush")]
async fn main() {
    // Load assets
    let assets = Assets::load().await;
    // Play menu music
    play_sound(&assets.menu_music, PlaySoundParams { looped: true, volume: 0.5 });

    let theme = VIKING_RUSH_THEME;
    let mut game_state = GameState::Menu;

    // Game objects
    let mut player = Player::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut projectiles: Vec<Projectile> = Vec::new();
    let mut upgrade_gates: Vec<UpgradeGate> = Vec::new();

    // Game stats
    let mut wave: u32 = 0;
    let mut score: u32 = 0;
    let mut enemies_to_spawn: u32 = 0;

    // Timers
    let mut spawn_timer = 0.0;
    let mut spawn_interval = 0.0;
    let mut attack_timer = 0.0;
    let attack_cooldown = 0.4;
    let mut gate_spawn_timer = 0.0;
    let gate_spawn_interval = 7.0;

    loop {
        let dt = get_frame_time();

        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) || touches().len() > 0 {
                    // start game
                    game_state = GameState::Playing;
                    stop_sound(&assets.menu_music);
                    play_sound(&assets.combat_music, PlaySoundParams { looped: true, volume: 0.5 });
                    player = Player::new();
                    enemies.clear();
                    projectiles.clear();
                    upgrade_gates.clear();

                    wave = 0;
                    score = 0;
                }

                draw_menu(&theme);
            }

            GameState::Playing => {
                // start a new wave if needed
                if enemies.is_empty() && enemies_to_spawn == 0 {
                    wave += 1;
                    enemies_to_spawn = start_wave(wave);
                    spawn_timer = 0.0;
                    spawn_interval = 30.0 / (enemies_to_spawn as f32);
                }

                // spawn enemies
                spawn_timer += dt;
                if spawn_timer >= spawn_interval && enemies_to_spawn > 0 {
                    enemies.push(spawn_enemy(wave));
                    spawn_timer = 0.0;
                    enemies_to_spawn -= 1;
                }

                // spawn upgrade gates
                gate_spawn_timer += dt;
                if gate_spawn_timer >= gate_spawn_interval && upgrade_gates.len() < 2 {
                    upgrade_gates.push(spawn_upgrade_gate());
                    gate_spawn_timer = 0.0;
                }

                // auto attack
                attack_timer += dt;
                if attack_timer >= attack_cooldown {
                    // find target
                    if let Some(target_pos) = find_closest_enemy(player.pos, &enemies) {
                        fire_projectiles(&player, target_pos, &mut projectiles);
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
                    game_state = GameState::GameOver;
                    stop_sound(&assets.combat_music);
                }

                // draw frame, player, enemies, gates and projectiles
                clear_background(Color::from_rgba(15, 20, 30, 255));
                player.draw();
                for enemy in &enemies {
                    enemy.draw();
                }
                for projectile in &projectiles {
                    projectile.draw();
                }
                for gate in &upgrade_gates {
                    gate.draw();
                }

                // HUD
                draw_hud(&player, wave, score);
            }

            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) || touches().len() > 0 {
                    game_state = GameState::Menu;
                    play_sound(&assets.menu_music, PlaySoundParams { looped: true, volume: 0.5 });
                }
                draw_game_over(score, wave);
            }
        }

        // wait for next frame
        next_frame().await
    }
}
