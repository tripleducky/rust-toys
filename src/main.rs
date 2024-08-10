use raylib::prelude::*;

const SCREEN_W: f32 = 640.0;
const SCREEN_H: f32 = 480.0;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let mut green_ball = Ball {
        position: Vector2::new(SCREEN_W / 2.0, SCREEN_H / 2.0),
        speed: 0.1,
        radius: 40.0,
        color: Color::GREEN,
    };

    while !rl.window_should_close() {
        /*----UPDATE----*/
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_D) { green_ball.position.x += green_ball.speed; }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) { green_ball.position.x -= green_ball.speed; }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S) { green_ball.position.y += green_ball.speed; }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W) { green_ball.position.y -= green_ball.speed; }

        /*----DRAW----*/
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);
        d.draw_circle_v(green_ball.position, green_ball.radius, green_ball.color);
    }
}

struct Ball {
    position: Vector2,
    speed: f32,
    radius: f32,
    color: Color,
}
