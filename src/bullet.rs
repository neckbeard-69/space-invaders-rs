use raylib::prelude::*;
pub struct Bullet {
    position: Vector2,
    size: f32,
    speed: f32,
}

impl Bullet {
    pub fn new(x: f32, y: f32) -> Self {
        Bullet {
            position: Vector2::new(x, y),
            size: 10.0,
            speed: 0.3,
        }
    }
    pub fn update(&mut self) {
        self.position.y -= self.speed;
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let rec = Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.size,
            height: self.size,
        };
        let roundness = 0.5;
        let segments = 8;
        d.draw_rectangle_rounded(rec, roundness, segments, Color::BLACK);
    }
    pub fn is_off_screen(&self) -> bool {
        self.position.y + self.size < 0.0
    }
}
