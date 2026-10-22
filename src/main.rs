use macroquad::prelude::*;

#[macroquad::main("yuri gagarin vs capitalism")]
async fn main() {
    let mut playerX: f32 = screen_width() / 2.0;
    let mut playerY: f32 = screen_height() / 2.0;
    let speed: f32 = 200.0;

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        // input
        if is_key_down(KeyCode::Y) || is_key_down(KeyCode::W) {
            playerY -= speed * dt;
        }
        if is_key_down(KeyCode::I) || is_key_down(KeyCode::S) {
            playerY += speed * dt;
        }
        if is_key_down(KeyCode::C) || is_key_down(KeyCode::A) {
            playerX -= speed * dt;
        }
        if is_key_down(KeyCode::E) || is_key_down(KeyCode::D) {
            playerX += speed * dt;
        }

        // stop you from going out of bounds
        playerX = clamp(playerX, 0.0, screen_width());
        playerY = clamp(playerY, 0.0, screen_height());

        // rendering
        draw_circle(playerX, playerY, 16.0, RED); // player

        next_frame().await;
    }
}
