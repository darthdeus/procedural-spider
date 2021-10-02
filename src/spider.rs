use crate::prelude::*;

pub const RESIZE_RATIO: f32 = 1.0;
const LEG_LENGTH: f32 = 64.0;

const BUTT_OFFSET: f32 = 32.0;
const BUTT_RADIUS: f32 = 32.0;

const COLOR: Color = VIOLET;
const R: f32 = 16.0;
const T: f32 = 8.0;

/// Total number of legs.
const LEG_COUNT: usize = 8;
/// Legs are placed on a circle, this is the extra "slots"
/// so legs don't end up being directly in front of the face.
const EXTRA_LEG_SPACING: usize = 2;
/// Degree of rotation per leg.
const LEG_DEGREE: f32 = std::f32::consts::TAU / (LEG_COUNT + EXTRA_LEG_SPACING) as f32;

// pub static mut USE_QUAT: bool = false;

// return face_dir;
// let angle = face_dir.angle_between(Vec2::new(0.0, 1.0));
// let angle = deg * i as f32;

#[derive(Default)]
pub struct Leg {
    // Where the leg anchors to the body
    origin_offset: Vec2,
    // Where we want the leg end to be
    target: Vec2,

    // Where it actually is
    end: Vec2,
    // Where the middle joint is
    mid: Vec2,

    lerp_mid: Vec2,
    lerp_end: Vec2,

    // Ideal direction of the leg, not taking spider's orientation into account
    ideal_leg_dir: Vec2,
}

pub struct Spider {
    pub pos: Vec2,
    face_dir: Vec2,
    legs: Vec<Leg>,

    pub debug_leg_angles: bool,
    pub debug_color_legs: bool,
}

// fn leg_origin_dir(face_dir: Vec2, i: usize) -> Vec2 {
//     let angle = LEG_DEGREE * (i + 1) as f32;
//
//     if unsafe { USE_QUAT } {
//         Mat3::from_rotation_z(angle).transform_vector2(face_dir)
//     } else {
//         // let total_angle = angle + face_dir.angle_between(Vec2::new(0.0, 1.0));
//         // let total_angle = angle + Vec2::new(1.0, 0.0).angle_between(face_dir);
//         let total_angle = angle + Vec2::new(1.0, 0.0).angle_between(face_dir);
//
//         Vec2::new(f32::cos(total_angle), f32::sin(total_angle)).normalize()
//     }
// }

impl Spider {
    pub fn new() -> Self {
        let pos = Vec2::new(
            screen_width() / 2.0 / RESIZE_RATIO,
            screen_height() / 2.0 / RESIZE_RATIO,
        );

        let face_dir = Vec2::new(0.0, 1.0);

        let mut legs = Vec::new();

        for i in 0..LEG_COUNT {
            // Leg's ideal target rotation, offsetby half the extra spacing.
            // In case of extra=2 it means there's no legs in the first and last slot.
            let angle = LEG_DEGREE * (i + EXTRA_LEG_SPACING / 2) as f32;

            let ideal_leg_dir = Mat3::from_rotation_z(angle)
                .transform_vector2(face_dir)
                .normalize();

            let mid_count = LEG_COUNT as f32 / 2.0;

            let origin_offset_len = (mid_count - (i + 1) as f32).abs() * 1.0;

            legs.push(Leg {
                origin_offset: ideal_leg_dir * origin_offset_len,
                target: pos + ideal_leg_dir * LEG_LENGTH * 2.0,

                ideal_leg_dir,

                ..Default::default()
            });
            // for (i, leg) in legs.iter_mut().enumerate() {
            //     leg.target = pos + leg_origin_dir(face_dir, i) * LEG_LENGTH * 2.0;
            // }
        }
        // let mut legs: [Leg; LEG_COUNT] = Default::default();

        Self {
            pos,
            face_dir,
            legs,

            debug_leg_angles: false,
            debug_color_legs: false,
        }
    }

    /// Returns a rotation transform in the direction the spider is facing.
    pub fn face_transform(&self) -> Mat3 {
        Mat3::from_rotation_z(Vec2::new(0.0, 1.0).angle_between(self.face_dir))
    }

    pub fn move_to(&mut self, new_pos: Vec2) {
        let new_face_dir = new_pos - self.pos;
        if new_face_dir.length() > 0.01 {
            self.face_dir = (0.9 * self.face_dir + 0.1 * new_face_dir.normalize()).normalize();
        }

        self.pos = new_pos;

        let face_transform = self.face_transform();

        for (i, leg) in self.legs.iter_mut().enumerate() {
            let leg_dir = leg.end - self.pos;
            // let ideal_leg_dir = leg_origin_dir(self.face_dir, i).normalize();
            let ideal_leg_dir = face_transform.transform_vector2(leg.ideal_leg_dir);

            if leg_dir.length() > 2.0 * LEG_LENGTH {
                leg.end = self.pos + ideal_leg_dir * LEG_LENGTH * 1.4;
            }

            if leg_dir.length() < 1.2 * LEG_LENGTH {
                leg.end = self.pos + ideal_leg_dir * LEG_LENGTH * 2.0;
            }

            let angle = leg_dir.angle_between(ideal_leg_dir).abs();

            if self.debug_leg_angles {
                root_ui().label(leg.end, &format!("angle {} = {:.2}", i, angle));
            }

            if angle > 0.9 {
                leg.end = self.pos + ideal_leg_dir * LEG_LENGTH * 1.6;
            }

            let target = (leg.end - self.pos).clamp_length(16.0, LEG_LENGTH * 2.0);

            let mut mid = target / 2.0;
            let norm = mid.perp();

            let d = |origin: Vec2, a: Vec2, b: Vec2| {
                f32::abs(a.length() - LEG_LENGTH + (b - a).length() - LEG_LENGTH)
            };

            let mut min_dist = d(self.pos, mid, target);
            let mut min_mid = mid;

            // TODO: use this instead of the `i` hack
            // let target_dir = (leg.target - self.pos).normalize();
            //
            // let a = target_dir;
            // let b = ideal_leg_dir;
            // let signed_area = f32::signum(a.x * b.y - a.y * b.x);
            // root_ui().label(leg.end, &format!("area {}", signed_area));

            for iter in 0..1000 {
                // TODO: use leg count
                let sign = if i < 4 { -1.0 } else { 1.0 };

                mid += sign * norm.normalize() * 0.1 * iter as f32;

                let new_dist = d(self.pos, mid, target);

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

        // let colors = [RED, GREEN, BLUE, YELLOW, VIOLET, BLACK, PINK, PURPLE, BEIGE];
        let colors = [YELLOW, ORANGE, RED, PURPLE, BLUE, GRAY, DARKGRAY, BLACK];

        for (i, leg) in self.legs.iter_mut().enumerate() {
            let lerp_speed = 0.5;
            leg.lerp_mid = leg.lerp_mid.lerp(leg.mid, lerp_speed);
            leg.lerp_end = leg.lerp_end.lerp(leg.end, lerp_speed);

            let color = if self.debug_color_legs {
                //             let mut color = Color::new(COLOR.r, COLOR.g, COLOR.b, COLOR.a);
                //
                //             color.r -= i as f32 / 20.0;
                //             color.g -= i as f32 / 20.0;
                //             color.b -= i as f32 / 20.0;

                colors[i]
            } else {
                COLOR
            };

            line(self.pos + leg.origin_offset, leg.lerp_mid, T, color);
            line(leg.lerp_mid, leg.lerp_end, T, color);

            draw_circle(leg.lerp_mid.x, leg.lerp_mid.y, 4.0, GREEN);
            draw_circle(leg.lerp_end.x, leg.lerp_end.y, 4.0, BLUE);
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
