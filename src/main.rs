use glam::*;
use macroquad::prelude::*;

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

    pub fn draw(&mut self, _ui: &mut egui::Ui) {
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

    let material = load_material(crt::VERTEX, crt::FRAGMENT, Default::default()).unwrap();

    let orig_pos = spider.pos;

    let move_min = spider.pos - Vec2::new(20.0, 20.0);
    let move_max = spider.pos + Vec2::new(20.0, 20.0);

    let mut i = 0.0;

    const MOVE_SPIDER_AUTO: bool = true;

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        i += 1.0 / 60.0;

        if MOVE_SPIDER_AUTO {
            spider.pos = orig_pos + Vec2::new(f32::sin(i), f32::cos(i)) * 80.0;
        } else {
            let mouse = mouse_position();
            spider.pos = Vec2::new(mouse.0, mouse.1);
        }

        f = f32::sin(f + 0.05);
        // clear_background(Color::new(1.0, f, 0.7, 1.0));
        // gl_use_material(material);

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

pub mod crt {
    pub const FRAGMENT: &str = r#"#version 100
        precision lowp float;
        varying vec4 color;
        varying vec2 uv;

        uniform sampler2D Texture;
        // https://www.shadertoy.com/view/XtlSD7

        vec2 CRTCurveUV(vec2 uv)
        {
            uv = uv * 2.0 - 1.0;
            vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
            uv = uv + uv * offset * offset;
            uv = uv * 0.5 + 0.5;
            return uv;
        }
        void DrawVignette( inout vec3 color, vec2 uv )
        {
            float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
            vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
            color *= vignette;
        }
        void DrawScanline( inout vec3 color, vec2 uv )
        {
            float iTime = 0.1;
            float scanline = clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
            float grille = 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );
            color *= scanline * grille * 1.2;
        }
        void main() {

            vec2 crtUV = CRTCurveUV(uv);

            vec3 res = texture2D(Texture, uv).rgb * color.rgb;

            if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
            {
                res = vec3(0.0, 0.0, 0.0);
            }
            DrawVignette(res, crtUV);
            DrawScanline(res, uv);
            gl_FragColor = vec4(res, 1.0);
        }
    "#;

    pub const VERTEX: &str = r#"#version 100
        attribute vec3 position;
        attribute vec2 texcoord;
        attribute vec4 color0;
        varying lowp vec2 uv;
        varying lowp vec4 color;
        uniform mat4 Model;
        uniform mat4 Projection;
        void main() {
            gl_Position = Projection * Model * vec4(position, 1);
            color = color0 / 255.0;
            uv = texcoord;
        }
    "#;
}
