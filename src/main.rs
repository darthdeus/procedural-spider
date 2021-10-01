use macroquad::prelude::*;

#[macroquad::main("Godot BITGUN (DEBUG)")]
async fn main() {
    let mut f = 0.1;

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        f = f32::sin(f + 0.05);

        // clear_background(Color::new(1.0, f, 0.7, 1.0));
        clear_background(Color::new(1.0, 0.6245, 0.7, 1.0));

        println!("f = {}", f);

        next_frame().await;
    }
}
