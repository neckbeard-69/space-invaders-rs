use crate::traits::Damageable;
use raylib::prelude::*;

pub struct Enemy {
    texture: Texture2D,
    position: Vector2,
    health: i16,
    damage_per_hit: i8,
    is_alive: bool,
}

impl Enemy {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        x: f32,
        y: f32,
        health: i16,
        damage_per_hit: i8,
    ) -> Self {
        let texture = rl
            .load_texture(&thread, "assets/LargeAlien.png")
            .expect("failed to load the player texture");

        Enemy {
            texture: texture,
            position: Vector2::new(x, y),
            health: health,
            damage_per_hit: damage_per_hit,
            is_alive: true,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.is_alive {
            d.draw_texture_ex(&self.texture, self.position, 0.0, 0.04, Color::RED);
        }
    }
}

impl Damageable for Enemy {
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
