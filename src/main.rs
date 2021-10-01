use glam::*;
use macroquad::prelude::*;

const LEG_LENGTH: f32 = 64.0;

struct Spider {
    pos: Vec2,
    legs: [Vec2; 8],
}

impl Spider {
    pub fn new() -> Self {
        let pos = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

        let mut legs: [Vec2; 8] = Default::default();

        for (i, leg) in legs.iter_mut().enumerate() {
            let deg = std::f32::consts::TAU / 8.0;
            *leg = pos
                + Vec2::new(f32::cos(deg * i as f32), f32::sin(deg * i as f32)).normalize()
                    * LEG_LENGTH * 2.0;
        }

        Self { pos, legs }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        let color = VIOLET;
        let r = 1.0;
        let t = 8.0;

        draw_circle(self.pos.x, self.pos.y, r, color);

        for leg in self.legs.iter() {
            let target = (*leg - self.pos).clamp_length(16.0, LEG_LENGTH * 2.0);

            // let leg = self.pos + dir;

            let mut mid = target / 2.0;
            let norm = mid.perp();

            let d = |a: Vec2, b: Vec2| f32::abs(a.length() - LEG_LENGTH + (b - a).length() - LEG_LENGTH);

            let mut min_dist = d(mid, target);
            let mut min_mid = mid;

            let mut distances = vec![];
            // let mut strings = vec![];

            for i in 0..1000 {
                mid += norm.normalize() * 0.1 * i as f32;

                let new_dist = d(mid, target);

                // strings.push(format!("{}, {}, {}", mid, ))

                if new_dist < min_dist {
                    min_dist = new_dist;
                    min_mid = mid;
                }

                distances.push(new_dist);
            }

            // ui.label(format!("Distances: {:.2?}", &distances[..10]));
            // mid.x += 16.0;

            let a = self.pos;
            let b = self.pos + min_mid;
            let c = self.pos + target;

            line(a, b, t, color);
            line(b, c, t, color);

            draw_circle(b.x, b.y, 4.0, GREEN);
            draw_circle(c.x, c.y, 4.0, BLUE);

            // draw_line(self.pos.x, self.pos.y, min_mid.x, min_mid.y, t, color);
            // draw_line(
            //     self.pos.x + min_mid.x,
            //     self.pos.y + min_mid.y,
            //     target.x,
            //     target.y,
            //     t,
            //     color,
            // );
            // draw_line(self.pos.x, self.pos.y, leg.x, leg.y, t, color);

            // draw_line(self.pos.x, self.pos.y, leg.x, leg.y, t, color);
        }
    }
}

fn line(pos: Vec2, dir: Vec2, thickness: f32, color: Color) {
    draw_line(pos.x, pos.y, dir.x, dir.y, thickness, color);
}

#[macroquad::main("Macroquad Spider")]
async fn main() {
    let mut f = 0.1;

    let mut spider = Spider::new();

    let orig_pos = spider.pos;

    let move_min = spider.pos - Vec2::new(20.0, 20.0);
    let move_max = spider.pos + Vec2::new(20.0, 20.0);

    let mut i = 0.0;

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        i += 1.0 / 60.0;

        spider.pos = orig_pos + Vec2::new(f32::sin(i), f32::cos(i)) * 20.0;

        f = f32::sin(f + 0.05);
        // clear_background(Color::new(1.0, f, 0.7, 1.0));
        clear_background(Color::new(1.0, 0.6245, 0.7, 1.0));

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("macroquaaad");
                ui.add(egui::Slider::new(&mut spider.pos.x, move_min.x..=move_max.x).text("x"));
                ui.add(egui::Slider::new(&mut spider.pos.y, move_min.y..=move_max.y).text("y"));

                spider.draw(ui);
            });
        });

        egui_macroquad::draw();

        // println!("f = {}", f);

        next_frame().await;
    }
}
