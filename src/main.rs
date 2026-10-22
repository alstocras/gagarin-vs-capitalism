use macroquad::prelude::*;

#[macroquad::main("yuri gagarin vs capitalism")]
async fn main() {
    let mut playerX: f32 = screen_width() / 2.0;
    let mut playerY: f32 = screen_height() / 2.0;
    let speed: f32 = 2.0;

    loop {
        clear_background(BLACK);

        // input
        if is_key_down(KeyCode::Y) || is_key_down(KeyCode::W) {
            playerY -= speed;
        }
        if is_key_down(KeyCode::I) || is_key_down(KeyCode::S) {
            playerY += speed;
        }
        if is_key_down(KeyCode::C) || is_key_down(KeyCode::A) {
            playerX -= speed;
        }
        if is_key_down(KeyCode::E) || is_key_down(KeyCode::D) {
            playerX += speed;
        }

        // rendering
        draw_circle(playerX, playerY, 16.0, RED); // player

        next_frame().await;
    }
}
