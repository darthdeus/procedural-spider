use macroquad::prelude::*;
use glam::*;

struct Spider {
    pos: Vec2,
}

impl Spider {
    pub fn new() -> Self {
        Self { pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0)  
    }}

    pub fn draw(&self) {
        let color = VIOLET;
        let r = 16.0;
        let t = 8.0;
        let leg = 32.0;

        draw_circle(self.pos.x, self.pos.y, r, color);

        line(self.pos, Vec2::new(leg, leg), t, color);
    }
}

fn line(pos: Vec2, dir: Vec2, thickness: f32, color: Color) {
    draw_line(pos.x, pos.y, pos.x + dir.x, pos.y + dir.y, thickness, color);
}

#[macroquad::main("Godot BITGUN (DEBUG)")]
async fn main() {
    let mut f = 0.1;

    let mut spider = Spider::new();

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        f = f32::sin(f + 0.05);

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("macroquaaad");
            });
        });

        // clear_background(Color::new(1.0, f, 0.7, 1.0));
        clear_background(Color::new(1.0, 0.6245, 0.7, 1.0));

        spider.draw();

        egui_macroquad::draw();

        // println!("f = {}", f);

        next_frame().await;
    }
}
