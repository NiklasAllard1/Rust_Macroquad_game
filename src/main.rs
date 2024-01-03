use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    loop {
        clear_background(DARKBLUE);
        next_frame().await
    }
}