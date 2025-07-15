use raylib::prelude::*;

use crate::{enemy::Enemy, player::Player};

mod bullet;
mod enemy;
mod player;
mod traits;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Space Invaders")
        .build();
    let width = get_monitor_width(0);
    let height = get_monitor_height(0);
    let offset: u8 = 100;
    rl.set_window_size(width - (offset as i32), height - (offset as i32));

    let mut player = Player::new(&mut rl, &thread, (width / 2) as f32, (height - 200) as f32);
    let enemy_count = 4;
    let mut enemies: Vec<Enemy> = Vec::with_capacity(enemy_count);
    for i in 0..enemy_count {
        enemies.push(Enemy::new(
            &mut rl,
            &thread,
            (10 * i + 10) as f32,
            (10 * i + 10) as f32,
            20,
            20,
        ));
    }

    while !rl.window_should_close() {
        player.enable_controls(&rl);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        player.draw(&mut d);
        for enemy in enemies.iter_mut() {
            enemy.draw(&mut d);
        }
        player.bullets.iter_mut().for_each(|b| b.update());

        // deallocate bullets when off screen
        // TODO: remove them when hitting a player too
        player.bullets.retain(|b| !b.is_off_screen());
        for bullet in &player.bullets {
            bullet.draw(&mut d);
        }
    }
}
