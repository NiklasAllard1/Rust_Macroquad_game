use macroquad::prelude::*;
use macroquad_particles::{self as particles, ColorCurve, Emitter, EmitterConfig};
use std::fs;

const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");

const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main(){
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

fn particle_explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.6,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 300.0,
        initial_velocity_randomness: 0.8,
        size: 3.0,
        size_randomness: 0.3,
        colors_curve: ColorCurve {
            start: RED,
            mid: ORANGE,
            end: RED,
        },
        ..Default::default()
    }
}
impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect{
        Rect{
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}
enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[macroquad::main("Mitt spel")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    
    rand::srand(miniquad::date::now() as u64);
    let mut squares = vec![];
    let mut bullets: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };
    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat").map_or(Ok(0), |i|i.parse::<u32>()).unwrap_or(0);
    let mut game_state = GameState::MainMenu;

    let mut direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
    
        MaterialParams {
            uniforms: vec![
                ("iResolution".to_owned(), UniformType::Float2),
                ("direction_modifier".to_owned(), UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let mut explosions: Vec<(Emitter, Vec2)> = vec![];

    loop {
        clear_background(BLACK);
        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();
    
    match game_state {
        GameState::MainMenu => {
            if is_key_pressed(KeyCode::Escape) {
                std::process::exit(0);
            }
            if is_key_pressed(KeyCode::Space) {
                squares.clear();
                bullets.clear();
                explosions.clear();
                circle.x = screen_width() / 2.0;
                circle.y = screen_height() / 2.0;
                score = 0;
                game_state = GameState::Playing;
            }
            let text = "Tryck på mellanslag";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                WHITE,
                );
            },
        
        GameState::Playing => {
            let delta_time = get_frame_time();
                if is_key_down(KeyCode::Right) {
                    circle.x += MOVEMENT_SPEED * delta_time;
                    direction_modifier += 0.05 * delta_time;
                }
                if is_key_down(KeyCode::Left) {
                    circle.x -= MOVEMENT_SPEED * delta_time;
                    direction_modifier -= 0.05 * delta_time;
                }
                if is_key_down(KeyCode::Down) {
                    circle.y += MOVEMENT_SPEED * delta_time;
                }
                if is_key_down(KeyCode::Up) {
                    circle.y -= MOVEMENT_SPEED * delta_time;
                }
                if is_key_pressed(KeyCode::Space) {
                    bullets.push(Shape {
                        x: circle.x,
                        y: circle.y,
                        speed: circle.speed * 2.0,
                        size: 5.0,
                        collided: false,
                    });
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
                    collided: false,
                });
            }
            //Movement
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }
            //remove shapes outside of screen
            squares.retain(|square| square.y < screen_height() + square.size);
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size /2.0);
            
            //Removes collided shapes
            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);

            // Remove old explosions
            explosions.retain(|(explosion, _)| explosion.config.emitting);

            //Check for collision
            if squares.iter().any(|square| circle.collides_with(square)) {
                if score == high_score {
                    fs::write("highscore.dat", high_score.to_string()).ok();
                }
                game_state = GameState::GameOver;
            }
            for square in squares.iter_mut() {
                for bullet in bullets.iter_mut() {
                    if bullet.collides_with(square) {
                        bullet.collided = true;
                        square.collided = true;
                        score += square.size.round() as u32;
                        high_score = high_score.max(score);
                        explosions.push((
                            Emitter::new(EmitterConfig {
                                amount: square.size.round() as u32 * 2,
                                ..particle_explosion()
                            }),
                            vec2(square.x, square.y),
                        ));
                    }
                }
            }
                //Draw everything
            for bullet in &bullets {
                draw_circle(bullet.x, bullet.y, bullet.size / 2.0, YELLOW);
            }
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
            for (explosions, coords) in explosions.iter_mut() {
                explosions.draw(*coords);
            }
            draw_text(
                format!("Poäng:{}", score).as_str(),
                10.0,
                35.0,
                25.0,
                WHITE,
            );
            
            let highscore_text = format!("High score: {}", high_score);
            let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
            draw_text(
                highscore_text.as_str(),
                screen_width() - text_dimensions.width - 10.0,
                35.0,
                25.0,
                WHITE,
            );
        },
        GameState::Paused => {
            if is_key_pressed(KeyCode::Escape) {
                game_state = GameState::Playing;
            }
            let text = "Pausad";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0 - 100.0,
                screen_height() / 2.0,
                100.0,
                RED,
            );
        },
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                
                }
                let text = "Game Over!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0 - 100.0,
                    screen_height() / 2.0,
                    100.0,
                    RED,
                );
            
                if score == high_score {
                    let text = "Congratulations, you have a new highscore!";
                    let text_dimensions = measure_text(text, None, 50, 1.0);
                    draw_text(
                        text,
                        screen_width() / 2.0 - text_dimensions.width / 2.0 + 240.0,
                        screen_height() / 3.0,
                        25.0,
                        YELLOW,
                    );
                }
            },
        }   

        next_frame().await

    }
}
