use glam::*;
use macroquad::prelude::*;

mod shaders;

const LEG_LENGTH: f32 = 64.0;

struct Spider {
    pos: Vec2,
    legs: [Vec2; 8],
}

fn leg_origin_dir(i: usize) -> Vec2 {
    let deg = std::f32::consts::TAU / 8.0;
    Vec2::new(f32::cos(deg * i as f32), f32::sin(deg * i as f32)).normalize()
}

impl Spider {
    pub fn new() -> Self {
        let pos = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

        let mut legs: [Vec2; 8] = Default::default();

        for (i, leg) in legs.iter_mut().enumerate() {
            *leg = pos + leg_origin_dir(i) * LEG_LENGTH * 2.0;
        }

        Self { pos, legs }
    }

    pub fn draw(&mut self) {
        let color = VIOLET;
        let r = 16.0;
        let t = 8.0;

        draw_circle(self.pos.x, self.pos.y, r, color);

        for (i, leg) in self.legs.iter_mut().enumerate() {
            let leg_dir = *leg - self.pos;

            if leg_dir.length() > 2.0 * LEG_LENGTH {
                *leg = self.pos + leg_origin_dir(i).normalize() * LEG_LENGTH * 1.4;
            }

            if leg_dir.length() < 1.2 * LEG_LENGTH {
                *leg = self.pos + leg_origin_dir(i).normalize() * LEG_LENGTH * 2.0;
            }

            let target = (*leg - self.pos).clamp_length(16.0, LEG_LENGTH * 2.0);

            let mut mid = target / 2.0;
            let norm = mid.perp();

            let d = |a: Vec2, b: Vec2| {
                f32::abs(a.length() - LEG_LENGTH + (b - a).length() - LEG_LENGTH)
            };

            let mut min_dist = d(mid, target);
            let mut min_mid = mid;

            for i in 0..1000 {
                mid += norm.normalize() * 0.1 * i as f32;

                let new_dist = d(mid, target);

                if new_dist < min_dist {
                    min_dist = new_dist;
                    min_mid = mid;
                }
            }

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

    let material = load_material(shaders::VERTEX, shaders::FRAGMENT, Default::default()).unwrap();

    let orig_pos = spider.pos;

    let move_min = spider.pos - Vec2::new(20.0, 20.0);
    let move_max = spider.pos + Vec2::new(20.0, 20.0);

    let mut i = 0.0;

    let mut use_shader = false;
    let mut auto_move_spider = true;

    let main_render_target = render_target(screen_width() as u32, screen_height() as u32);
    main_render_target.texture.set_filter(FilterMode::Nearest);

    const NICE_PINK: Color = Color::new(1.0, 0.6245, 0.7, 1.0);

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        i += 1.0 / 60.0;

        if auto_move_spider {
            spider.pos = orig_pos + Vec2::new(f32::sin(i), f32::cos(i)) * 80.0;
        } else {
            let mouse = mouse_position();
            spider.pos = Vec2::new(mouse.0, mouse.1);
        }

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("macroquaaad");
                ui.add(egui::Slider::new(&mut spider.pos.x, move_min.x..=move_max.x).text("x"));
                ui.add(egui::Slider::new(&mut spider.pos.y, move_min.y..=move_max.y).text("y"));

                ui.checkbox(&mut use_shader, "Use shader:");
                ui.checkbox(&mut auto_move_spider, "Auto move spider:");
            });
        });

        // const SCR_W: f32 = 100.0;
        // const SCR_H: f32 = 60.0;
        //
        let SCR_W = screen_width();
        let SCR_H = screen_height();

        set_camera(&Camera2D {
            zoom: vec2(1.0 / SCR_W * 2.0, -1.0 / SCR_H * 2.0),
            target: vec2(SCR_W / 2.0, SCR_H / 2.0),
            render_target: Some(main_render_target),
            ..Default::default()
        });

        clear_background(NICE_PINK);

        spider.draw();

        set_default_camera();

        if use_shader {
            gl_use_material(material);
        }


        draw_texture_ex(
            main_render_target.texture,
            0.0,
            0.0,
            NICE_PINK,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        gl_use_default_material();
        egui_macroquad::draw();

        // clear_background(Color::new(1.0, 0.6245, 0.7, 1.0));

        // draw_texture(main_render_target.texture, 0.0, 0.0, NICE_PINK);
        // clear_background(NICE_PINK);
        // draw_texture_ex()
        // let screen_data: Image = get_screen_data();

        // println!("f = {}", f);

        next_frame().await;
    }
}
