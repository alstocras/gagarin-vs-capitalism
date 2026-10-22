use macroquad::prelude::*;

struct GameObject {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main("yuri gagarin vs capitalism")]
async fn main() {
    let mut enemies = vec![];

    let mut player = GameObject {
        size: 32.0,
        speed: 200.0,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    // seed random generator
    rand::srand(miniquad::date::now() as u64);

    loop {
        // for smooth movement (how long since last frame)
        let dt = get_frame_time();
        clear_background(BLACK);

        // input
        if is_key_down(KeyCode::Y) || is_key_down(KeyCode::W) {
            player.y -= player.speed * dt;
        }
        if is_key_down(KeyCode::I) || is_key_down(KeyCode::S) {
            player.y += player.speed * dt;
        }
        if is_key_down(KeyCode::C) || is_key_down(KeyCode::A) {
            player.x -= player.speed * dt;
        }
        if is_key_down(KeyCode::E) || is_key_down(KeyCode::D) {
            player.x += player.speed * dt;
        }

        // stop you from going out of bounds
        player.x = clamp(player.x, 0.0, screen_width());
        player.y = clamp(player.y, 0.0, screen_height());

        // add enemies
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(8.0, 128.0);
            enemies.push(GameObject {
                size,
                speed: rand::gen_range(12.0, 200.0),
                x: screen_width() / 2.0,
                y: -size,
            });
        }

        // move & draw enemies
        for enemy in &mut enemies {
            enemy.y += enemy.speed * dt;
        }

        // delete if offscreen
        enemies.retain(|enemy| enemy.y < screen_height() + enemy.size);

        // rendering
        draw_circle(player.x, player.y, 16.0, RED); // player
        for enemy in &enemies {
            // enemies
            draw_rectangle(enemy.x, enemy.y, enemy.size, enemy.size, BLUE);
        }

        next_frame().await;
    }
}
