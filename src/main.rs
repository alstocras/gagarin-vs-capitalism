use macroquad::prelude::*;
use macroquad_particles as particles;
use macroquad_particles::*;
use std::fs::*;

struct GameObject {
    height: f32,
    width: f32,
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
            x: self.x - self.width / 2.0,
            y: self.y - self.height / 2.0,
            w: self.width,
            h: self.height,
        }
    }
}

fn particleExplosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 1.0,
        lifetime_randomness: 0.5,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 300.0,
        initial_velocity_randomness: 200.0,
        size: 3.0,
        size_randomness: 1.0,
        colors_curve: ColorCurve {
            start: BLUE,
            mid: WHITE,
            end: RED,
        },
        ..Default::default()
    }
}

fn returnExhaust() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: false,
        lifetime: 2.0,
        explosiveness: 0.5,
        lifetime_randomness: 0.4,
        amount: 200,
        initial_direction: vec2(0.0, 1.0),
        initial_direction_spread: 0.5,
        gravity: vec2(0.0, 50.0),
        initial_velocity: 100.0,
        initial_velocity_randomness: 0.5,
        size: 2.0,
        colors_curve: ColorCurve {
            start: WHITE,
            mid: ORANGE,
            end: RED,
        },
        ..Default::default()
    }
}

enum GameState {
    Menu,
    Game,
    Paused,
    Defeat,
}

#[macroquad::main("yuri gagarin vs capitalism")]
async fn main() {
    let jbmGameFont = load_ttf_font("assets/fonts/jbm.ttf").await.unwrap();
    set_pc_assets_folder("assets");
    set_default_font(jbmGameFont);

    let playerSprite: Texture2D = load_texture("sprites/vostok.png")
        .await
        .expect("failed to load");
    playerSprite.set_filter(FilterMode::Nearest);
    let mut playerRotation: f32 = 0.0;

    let enemySprite: Texture2D = load_texture("sprites/enemy.png")
        .await
        .expect("failed to load");
    enemySprite.set_filter(FilterMode::Nearest);

    let bulletSprite: Texture2D = load_texture("sprites/bullet.png")
        .await
        .expect("failed to load");
    bulletSprite.set_filter(FilterMode::Nearest);

    build_textures_atlas();

    let mut exhaustEmitter = Emitter::new(returnExhaust());

    let mut enemies = vec![];
    let mut bullets: Vec<GameObject> = vec![];
    let mut explosions: Vec<(Emitter, Vec2)> = vec![];

    let mut shotTime: f64 = get_time();
    let mut score: u32 = 0;
    // filereading stuff, basically reads the file and converts to u32
    let mut highScore: u32 = read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);

    let mut player = GameObject {
        height: playerSprite.height(),
        width: playerSprite.width(),
        speed: 200.0,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        colour: RED,
        hitByBullet: false,
    };

    let mut gameState = GameState::Menu;

    let enemyColours = [BLUE, GREEN, YELLOW, ORANGE, WHITE];

    // seed random generator
    rand::srand(miniquad::date::now() as u64);

    loop {
        match gameState {
            GameState::Menu => {
                // quit
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                // play
                if is_key_pressed(KeyCode::Space) {
                    score = 0;
                    enemies.clear();
                    bullets.clear();
                    player.x = screen_width() / 2.0;
                    player.y = screen_height() / 2.0;
                    explosions.clear();
                    gameState = GameState::Game;
                }
                let helperText = "press space to play, esc to quit";
                let helperTextDimen = measure_text(helperText, None, 35, 1.0);
                draw_text(
                    helperText,
                    screen_width() / 2.0 - helperTextDimen.width / 2.0,
                    screen_height() / 2.0 + helperTextDimen.height / 2.0,
                    35.0,
                    WHITE,
                );

                // TITLE
                let title = "gagarin vs capitalism";
                let titleDimen = measure_text(title, None, 70, 1.0);
                draw_text(
                    title,
                    screen_width() / 2.0 - titleDimen.width / 2.0,
                    screen_height() / 2.0 - helperTextDimen.height - 25.0,
                    70.0,
                    RED,
                );
            }
            GameState::Game => {
                // for smooth movement (how long since last frame)
                let dt = get_frame_time();
                clear_background(BLACK);

                // input
                if is_key_down(KeyCode::Y) || is_key_down(KeyCode::W) {
                    player.y -= player.speed * dt;
                    playerRotation = 0.0;
                }
                if is_key_down(KeyCode::I) || is_key_down(KeyCode::S) {
                    player.y += player.speed * dt;
                    playerRotation = std::f32::consts::PI;
                }
                if is_key_down(KeyCode::C) || is_key_down(KeyCode::A) {
                    player.x -= player.speed * dt;
                    playerRotation = -std::f32::consts::PI / 2.0;
                }
                if is_key_down(KeyCode::E) || is_key_down(KeyCode::D) {
                    player.x += player.speed * dt;
                    playerRotation = std::f32::consts::PI / 2.0;
                }
                if !is_any_key_down() {
                    playerRotation = 0.0;
                }

                // shooting
                if is_key_down(KeyCode::Space) {
                    if (get_time() - shotTime) > 0.25 {
                        bullets.push(GameObject {
                            x: player.x,
                            y: player.y,
                            speed: player.speed * 2.0,
                            width: 5.0,
                            height: 5.0,
                            colour: PURPLE,
                            hitByBullet: false,
                        });
                        shotTime = get_time();
                    }
                }

                // pausing
                if is_key_pressed(KeyCode::Escape) {
                    gameState = GameState::Paused;
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
                        width: enemySprite.width(),
                        height: enemySprite.height(),
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
                        if score >= highScore {
                            write("highscore.dat", highScore.to_string()).ok();
                        }
                        gameState = GameState::Defeat;
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
                enemies.retain(|enemy| enemy.y < screen_height() + enemy.height / 2.0);
                bullets.retain(|bullet| bullet.y > 0.0 - bullet.height / 2.0);

                // delete if shot
                enemies.retain(|enemy| !enemy.hitByBullet);
                bullets.retain(|bullet| !bullet.hitByBullet);

                // delete when done
                explosions.retain(|(explosion, _)| explosion.config.emitting);

                // collisions
                for bullet in &mut bullets {
                    for enemy in &mut enemies {
                        if bullet.checkCollision(enemy) {
                            bullet.hitByBullet = true;
                            enemy.hitByBullet = true;
                            // bigger and faster = more explosion
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: (enemy.height.round() + enemy.speed.round()) as u32 * 2,
                                    ..particleExplosion()
                                }),
                                vec2(enemy.x, enemy.y),
                            ));
                            score += (enemy.height.round() + enemy.speed.round()) as u32;
                            highScore = highScore.max(score);
                        }
                    }
                }

                // rendering

                // enemies
                for enemy in &enemies {
                    draw_texture_ex(
                        &enemySprite,
                        enemy.x - enemySprite.width() / 2.0,
                        enemy.y - enemySprite.height() / 2.0,
                        enemy.colour,
                        DrawTextureParams {
                            rotation: 0.0,
                            pivot: None,
                            ..Default::default()
                        },
                    );
                }
                // bullets
                for bullet in &bullets {
                    draw_circle(bullet.x, bullet.y, bullet.height, PURPLE);
                    draw_texture(
                        &bulletSprite,
                        bullet.x - bulletSprite.width() / 2.0,
                        bullet.y - bulletSprite.height() / 2.0,
                        WHITE,
                    )
                }
                // player
                exhaustEmitter.draw(vec2(player.x, player.y));
                draw_texture_ex(
                    &playerSprite,
                    player.x - playerSprite.width() / 2.0,
                    player.y - playerSprite.height() / 2.0,
                    WHITE,
                    DrawTextureParams {
                        rotation: playerRotation,
                        pivot: None,
                        ..Default::default()
                    },
                );

                // effects
                for (explosion, coords) in &mut explosions {
                    explosion.draw(*coords);
                }

                // render scores
                draw_text(format!("score: {}", score).as_str(), 10.0, 35.0, 25.0, RED);

                let highScoreText = format!("high score: {}", highScore);
                let highScoreDimensions = measure_text(highScoreText.as_str(), None, 25, 1.0);
                draw_text(
                    highScoreText.as_str(),
                    screen_width() - highScoreDimensions.width,
                    35.0,
                    25.0,
                    RED,
                );
            }
            GameState::Paused => {
                let message = "game paused; esc to go back";
                let textDimensions = measure_text(message, None, 50, 1.0);
                draw_text(
                    message,
                    screen_width() / 2.0 - textDimensions.width / 2.0,
                    screen_height() / 2.0 + textDimensions.height / 2.0,
                    50.0,
                    RED,
                );
                if is_key_pressed(KeyCode::Escape) {
                    gameState = GameState::Game;
                }
            }
            GameState::Defeat => {
                let message = "capitalism has defeated you. space to return to the menu.";
                let textDimensions = measure_text(message, None, 30, 1.0);
                draw_text(
                    message,
                    screen_width() / 2.0 - textDimensions.width / 2.0,
                    screen_height() / 2.0 + textDimensions.height / 2.0,
                    30.0,
                    RED,
                );
                let scoreInfo = format!("high score: {}", highScore);
                let scoreMeasure = measure_text(scoreInfo.as_str(), None, 25, 1.0);
                draw_text(
                    scoreInfo.as_str(),
                    screen_width() / 2.0 - scoreMeasure.width / 2.0,
                    screen_height() / 2.0 + 2.0 * scoreMeasure.height,
                    25.0,
                    RED,
                );
                if is_key_pressed(KeyCode::Space) {
                    score = 0;
                    enemies.clear();
                    bullets.clear();
                    player.x = screen_width() / 2.0;
                    player.y = screen_height() / 2.0;
                    gameState = GameState::Menu;
                }
            }
        }

        next_frame().await;
    }
}
