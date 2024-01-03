use macroquad::prelude::*;

#[macroquad::main("Mitt spel")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() /2.0;
    loop {
        clear_background(DARKBLUE);
        if is_key_down(KeyCode::Right) {
            x += 2.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 2.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 2.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 2.0;
        }

        draw_circle(x, y, 18.0, RED);

        next_frame().await
    }
}