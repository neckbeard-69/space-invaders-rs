use raylib::prelude::*;

use crate::player::Player;

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
    let player = Player::new(&mut rl, &thread, 200.0, 200.0);
    rl.set_window_size(width - (offset as i32), height - (offset as i32));

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        player.draw(&mut d);
        d.clear_background(Color::WHITE);
    }
}
