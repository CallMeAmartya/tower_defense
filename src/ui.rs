use crate::config::Theme;
use crate::player::Player;
use macroquad::prelude::*;

pub fn draw_menu(theme: &Theme) {
    clear_background(Color::from_rgba(15, 20, 30, 255));

    // Title
    let title = theme.game_title;
    let title_width = measure_text(title, None, 64, 1.0).width;
    draw_text(
        title,
        screen_width() / 2.0 - title_width / 2.0,
        screen_height() / 3.0,
        64.0,
        theme.player_color,
    );

    // Subtitle
    let subtitle = "Defend the Arctic!";
    let sub_width = measure_text(subtitle, None, 28, 1.0).width;
    draw_text(
        subtitle,
        screen_width() / 2.0 - sub_width / 2.0,
        screen_height() / 3.0 + 50.0,
        28.0,
        WHITE,
    );

    // Pulsing "Press SPACE" text
    let pulse = (get_time() * 2.0).sin() as f32 * 0.3 + 0.7;
    let instruction = "Press SPACE to start";
    let inst_width = measure_text(instruction, None, 24, 1.0).width;
    draw_text(
        instruction,
        screen_width() / 2.0 - inst_width / 2.0,
        screen_height() * 0.6,
        24.0,
        Color::new(1.0, 1.0, 1.0, pulse),
    );

    // Controls
    draw_text(
        "WASD to move",
        screen_width() / 2.0 - 60.0,
        screen_height() * 0.75,
        18.0,
        GRAY,
    );
    draw_text(
        "Run through gates for upgrades!",
        screen_width() / 2.0 - 120.0,
        screen_height() * 0.75 + 25.0,
        18.0,
        GRAY,
    );
}

pub fn draw_game_over(score: u32, wave: u32) {
    // Darken background
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::from_rgba(0, 0, 0, 200),
    );

    // Game Over text
    let title = "GAME OVER";
    let title_width = measure_text(title, None, 64, 1.0).width;
    draw_text(
        title,
        screen_width() / 2.0 - title_width / 2.0,
        screen_height() / 3.0,
        64.0,
        RED,
    );

    // Stats
    draw_text(
        &format!("Score: {}", score),
        screen_width() / 2.0 - 80.0,
        screen_height() / 2.0,
        32.0,
        WHITE,
    );
    draw_text(
        &format!("Waves Survived: {}", wave.saturating_sub(1)),
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 + 40.0,
        24.0,
        LIGHTGRAY,
    );

    // Restart prompt
    let pulse = (get_time() * 2.0).sin() as f32 * 0.3 + 0.7;
    draw_text(
        "Press SPACE to restart",
        screen_width() / 2.0 - 120.0,
        screen_height() * 0.75,
        24.0,
        Color::new(1.0, 1.0, 1.0, pulse),
    );
}

pub fn draw_hud(player: &Player, wave: u32, score: u32) {
    // Wave & Score
    draw_text(&format!("Wave {}", wave), 20.0, 35.0, 32.0, WHITE);
    draw_text(&format!("Score: {}", score), 20.0, 60.0, 20.0, LIGHTGRAY);

    // Player stats (top right)
    let x = screen_width() - 180.0;
    draw_text(
        &format!("DMG: {:.0}", player.damage),
        x,
        35.0,
        18.0,
        Color::from_rgba(255, 150, 100, 255),
    );
    draw_text(
        &format!("Shots: {}", player.projectile_count),
        x,
        55.0,
        18.0,
        Color::from_rgba(100, 200, 255, 255),
    );
    draw_text(
        &format!("Speed: {:.0}", player.speed),
        x,
        75.0,
        18.0,
        Color::from_rgba(255, 255, 100, 255),
    );

    // Health bar (bottom of screen)
    let bar_width = 200.0;
    let bar_height = 20.0;
    let bar_x = screen_width() / 2.0 - bar_width / 2.0;
    let bar_y = screen_height() - 40.0;
    let health_pct = (player.health / player.max_health).clamp(0.0, 1.0);

    draw_rectangle(
        bar_x,
        bar_y,
        bar_width,
        bar_height,
        Color::from_rgba(40, 40, 40, 200),
    );

    let health_color = if health_pct > 0.5 {
        GREEN
    } else if health_pct > 0.25 {
        ORANGE
    } else {
        RED
    };
    draw_rectangle(
        bar_x,
        bar_y,
        bar_width * health_pct,
        bar_height,
        health_color,
    );
    draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, WHITE);

    // Health text
    draw_text(
        &format!("{:.0}/{:.0}", player.health, player.max_health),
        bar_x + bar_width / 2.0 - 30.0,
        bar_y + 15.0,
        16.0,
        WHITE,
    );
}
