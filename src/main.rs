use macroquad::prelude::*;
use std::fs::*;

struct GameObject {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    colour: Color,
    hitByBullet: bool,
}

impl GameObject {
    fn checkCollision(&self, other: &Self) -> bool {
        self.makeRect().overlaps(&other.makeRect())
    }

    fn makeRect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("yuri gagarin vs capitalism")]
async fn main() {
    let jbmGameFont = load_ttf_font("assets/fonts/jbm.ttf").await.unwrap();
    set_default_font(jbmGameFont);
    let mut enemies = vec![];
    let mut bullets: Vec<GameObject> = vec![];
    let mut shotTime: f64 = get_time();
    let mut score: u32 = 0;

    let mut player = GameObject {
        size: 32.0,
        speed: 200.0,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        colour: RED,
        hitByBullet: false,
    };

    let mut isGameOver: bool = false;

    let enemyColours = [BLUE, GREEN, YELLOW, ORANGE, WHITE];

    // seed random generator
    rand::srand(miniquad::date::now() as u64);

    loop {
        if !isGameOver {
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

            // shooting
            if is_key_down(KeyCode::Space) {
                if (get_time() - shotTime) > 0.25 {
                    bullets.push(GameObject {
                        x: player.x,
                        y: player.y,
                        speed: player.speed * 2.0,
                        size: 5.0,
                        colour: PURPLE,
                        hitByBullet: false,
                    });
                    shotTime = get_time();
                } else {
                    continue;
                }
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
                    hitByBullet: false,
                });
            }

            // collisions
            for e in &enemies {
                if player.checkCollision(e) {
                    println!("collide!!");
                    isGameOver = true;
                };
            }

            // move enemies
            for enemy in &mut enemies {
                enemy.y += enemy.speed * dt;
            }
            // move bullets
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * dt;
            }

            // delete if offscreen
            enemies.retain(|enemy| enemy.y < screen_height() + enemy.size / 2.0);
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

            // delete if shot
            enemies.retain(|enemy| !enemy.hitByBullet);
            bullets.retain(|bullet| !bullet.hitByBullet);

            // collisions
            for bullet in &mut bullets {
                for enemy in &mut enemies {
                    if bullet.checkCollision(enemy) {
                        bullet.hitByBullet = true;
                        enemy.hitByBullet = true;
                    }
                }
            }

            // rendering

            // enemies
            for enemy in &enemies {
                draw_rectangle(
                    enemy.x - enemy.size / 2.0,
                    enemy.y - enemy.size / 2.0,
                    enemy.size,
                    enemy.size,
                    enemy.colour,
                );
            }
            // bullets
            for bullet in &bullets {
                draw_circle(bullet.x, bullet.y, bullet.size, PURPLE);
            }
            draw_circle(player.x, player.y, 16.0, RED); // player\
        }

        if isGameOver {
            let message = "capitalism has defeated you, товарищ. space to try again.";
            let textDimensions = measure_text(message, None, 30, 1.0);
            draw_text(
                message,
                screen_width() / 2.0 - textDimensions.width / 2.0,
                screen_height() / 2.0 + textDimensions.height / 2.0,
                30.0,
                RED,
            );
        }

        if isGameOver && is_key_pressed(KeyCode::Space) {
            enemies.clear();
            bullets.clear();
            player.x = screen_width() / 2.0;
            player.y = screen_height() / 2.0;
            isGameOver = false;
        }

        next_frame().await;
    }
}
