use macroquad::prelude::*;

#[macroquad::main("yuri gagarin vs capitalism")]
async fn main() {
    loop {
        clear_background(RED);

        next_frame().await;
    }
}
