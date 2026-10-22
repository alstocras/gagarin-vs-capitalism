use macroquad::prelude::*;

struct GameObject {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    colour: Color,
}

#[macroquad::main("yuri gagarin vs capitalism")]
async fn main() {
    let mut enemies = vec![];

    let mut player = GameObject {
        size: 32.0,
        speed: 200.0,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        colour: RED,
    };

    let enemyColours = [BLUE, GREEN, YELLOW, ORANGE, WHITE];

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
            let size = rand::gen_range(16.0, 64.0);
            let colourIndex = rand::gen_range(0, 5) as usize;
            let genColour = enemyColours[colourIndex];
            enemies.push(GameObject {
                size,
                speed: rand::gen_range(20.0, 200.0),
                x: rand::gen_range(0.0, screen_width()),
                y: -size,
                colour: genColour,
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
            draw_rectangle(enemy.x, enemy.y, enemy.size, enemy.size, enemy.colour);
        }

        next_frame().await;
    }
}
