use crate::prelude::*;

pub const RESIZE_RATIO: f32 = 1.0;
const LEG_LENGTH: f32 = 64.0;

const BUTT_OFFSET: f32 = 32.0;
const BUTT_RADIUS: f32 = 32.0;

const COLOR: Color = VIOLET;
const R: f32 = 16.0;
const T: f32 = 8.0;

#[derive(Default)]
pub struct Leg {
    // Where we want the leg to be
    target: Vec2,

    // Where it actually is
    end: Vec2,
    // Where the middle joint is
    mid: Vec2,
}

pub struct Spider {
    pub pos: Vec2,
    face_dir: Vec2,
    legs: [Leg; 8],
}

fn leg_origin_dir(i: usize) -> Vec2 {
    let deg = std::f32::consts::TAU / 8.0;
    Vec2::new(f32::cos(deg * i as f32), f32::sin(deg * i as f32)).normalize()
}

impl Spider {
    pub fn new() -> Self {
        let pos = Vec2::new(
            screen_width() / 2.0 / RESIZE_RATIO,
            screen_height() / 2.0 / RESIZE_RATIO,
        );

        let mut legs: [Leg; 8] = Default::default();

        for (i, leg) in legs.iter_mut().enumerate() {
            leg.target = pos + leg_origin_dir(i) * LEG_LENGTH * 2.0;
        }

        Self {
            pos,
            face_dir: Vec2::new(0.0, 1.0),
            legs,
        }
    }

    pub fn move_to(&mut self, new_pos: Vec2) {
        self.face_dir = (new_pos - self.pos).normalize();
        self.pos = new_pos;

        for (i, leg) in self.legs.iter_mut().enumerate() {
            let leg_dir = leg.end - self.pos;

            if leg_dir.length() > 2.0 * LEG_LENGTH {
                leg.end = self.pos + leg_origin_dir(i).normalize() * LEG_LENGTH * 1.4;
            }

            if leg_dir.length() < 1.2 * LEG_LENGTH {
                leg.end = self.pos + leg_origin_dir(i).normalize() * LEG_LENGTH * 2.0;
            }

            let target = (leg.end - self.pos).clamp_length(16.0, LEG_LENGTH * 2.0);

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

            leg.mid = self.pos + min_mid;
            leg.end = self.pos + target;
        }
    }

    pub fn draw(&mut self) {
        draw_circle(self.pos.x, self.pos.y, R, COLOR);
        draw_circle(
            self.pos.x - self.face_dir.x * BUTT_OFFSET,
            self.pos.y - self.face_dir.y * BUTT_OFFSET,
            BUTT_RADIUS,
            COLOR,
        );

        for leg in self.legs.iter() {
            line(self.pos, leg.mid, T, COLOR);
            line(leg.mid, leg.end, T, COLOR);

            draw_circle(leg.mid.x, leg.mid.y, 4.0, GREEN);
            draw_circle(leg.end.x, leg.end.y, 4.0, BLUE);
        }

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

fn line(pos: Vec2, dir: Vec2, thickness: f32, color: Color) {
    draw_line(pos.x, pos.y, dir.x, dir.y, thickness, color);
}
