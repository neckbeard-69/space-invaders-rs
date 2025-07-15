use crate::{
    bullet::{self, Bullet},
    traits::Damageable,
};
use raylib::prelude::*;

pub struct Player {
    texture: Texture2D,
    position: Vector2,
    health: i16,
    damage_per_hit: i8,
    is_alive: bool,
    pub bullets: Vec<Bullet>,
}

impl Player {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, x: f32, y: f32) -> Self {
        let texture = rl
            .load_texture(&thread, "assets/LargeAlien.png")
            .expect("failed to load the player texture");

        let width = texture.width as f32; // for multiplication precision, otherwise I'll get a 0
        let scale: f32 = 0.1;
        Player {
            texture: texture,
            position: Vector2::new(x - ((width * scale) as i32) as f32, y), // yeah, some dark
            // magic
            health: 100,
            damage_per_hit: 30, // not sure how much I should make it yet
            is_alive: true,
            bullets: Vec::new(),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_ex(
            &self.texture,
            self.position,
            0.0,
            0.1, /* scale */
            Color::WHITE,
        );
    }

    fn fire_bullet(&mut self) {
        let bullet = Bullet::new(self.position.x, self.position.y + 40.0);
        self.bullets.push(bullet);
    }

    pub fn enable_controls(&mut self, rl: &RaylibHandle) {
        let speed = 20;
        if rl.is_key_pressed(KeyboardKey::KEY_A) || rl.is_key_pressed_repeat(KeyboardKey::KEY_A) {
            self.position.x -= speed as f32;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_D) || rl.is_key_pressed_repeat(KeyboardKey::KEY_D) {
            self.position.x += speed as f32;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.fire_bullet();
        }
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
