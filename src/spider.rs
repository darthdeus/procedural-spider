use crate::prelude::*;

pub const RESIZE_RATIO: f32 = 1.0;
const LEG_LENGTH: f32 = 64.0;

pub struct Spider {
    pub pos: Vec2,
    legs: [Vec2; 8],
}

fn leg_origin_dir(i: usize) -> Vec2 {
    let deg = std::f32::consts::TAU / 8.0;
    Vec2::new(f32::cos(deg * i as f32), f32::sin(deg * i as f32)).normalize()
}

impl Spider {
    pub fn new() -> Self {
        let pos = Vec2::new(screen_width() / 2.0 / RESIZE_RATIO, screen_height() / 2.0 / RESIZE_RATIO);

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
