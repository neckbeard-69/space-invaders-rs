use crate::traits::Damageable;
use raylib::prelude::*;

pub struct Player {
    texture: Texture2D,
    position: Vector2,
    health: i16,
    damage_per_hit: i8,
    is_alive: bool,
}

impl Player {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, x: f32, y: f32) -> Self {
        let texture = rl
            .load_texture(&thread, "assets/LargeAlien.png")
            .expect("failed to load the player texture");

        Player {
            texture: texture,
            position: Vector2::new(x, y),
            health: 100,
            damage_per_hit: 30, // not sure how much I should make it yet
            is_alive: true,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_ex(&self.texture, self.position, 0.0, 0.1, Color::WHITE);
    }
}

impl Damageable for Player {
    fn take_damage(&mut self, damage_amount: i16) {
        self.health -= damage_amount;
        if self.health <= 0 {
            self.is_alive = false;
        }
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}
