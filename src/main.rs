use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}
#[macroquad::main("Mitt spel")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    
    rand::srand(miniquad::date::now() as u64);
    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    loop {
        clear_background(DARKBLUE);
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Right) {
            circle.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= MOVEMENT_SPEED * delta_time;
        }

        circle.x = circle.x.min(screen_width()).max(0.0);
        circle.y = circle.y.min(screen_height()).max(0.0);
            
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            });
        }

        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        squares.retain(|square| square.y < screen_height() + square.size);

        draw_circle(circle.x, circle.y, circle.size / 2.0, RED);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                PURPLE,
            );
        }
        next_frame().await
    }
}