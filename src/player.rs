use raylib::prelude::*;

pub struct Player {
    texture: Texture2D,
    position: Vector2,
}

impl Player {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, x: f32, y: f32) -> Self {
        let texture = rl
            .load_texture(&thread, "assets/LargeAlien.png")
            .expect("failed to load the player texture");

        Player {
            texture: texture,
            position: Vector2::new(x, y),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_ex(&self.texture, self.position, 0.0, 0.1, Color::BLACK);
    }
}
