use crate::prelude::*;

const LEG_LENGTH: f32 = 64.0;

const MOVE_SPEED: f32 = 30.0;
const BUTT_OFFSET: f32 = 32.0;
const BUTT_RADIUS: f32 = 32.0;

const BODY_COLOR: Color = Color::new(0.8, 0.4245, 0.4, 1.0);
const PLAYER_BODY_COLOR: Color = Color::new(0.7, 0.3245, 0.3, 1.0);
const R: f32 = 16.0;
const T: f32 = 8.0;

/// Total number of legs.
const LEG_COUNT: usize = 8;
/// Legs are placed on a circle, this is the extra "slots"
/// so legs don't end up being directly in front of the face.
const EXTRA_LEG_SPACING: usize = 2;
/// Degree of rotation per leg.
const LEG_DEGREE: f32 = std::f32::consts::TAU / (LEG_COUNT + EXTRA_LEG_SPACING) as f32;

pub static mut DEBUG_AI_LABELS: bool = false;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SpiderType {
    Player,
    Left,
    Right,
}

#[derive(Default, Debug)]
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

#[derive(Debug)]
pub struct Spider {
    pub pos: Vec2,
    velocity: Vec2,
    spider_type: SpiderType,

    pub scale: f32,

    face_dir: Vec2,
    legs: Vec<Leg>,

    pub max_leg_angle: f32,

    pub debug_leg_angles: bool,
    pub debug_color_legs: bool,
    pub debug_draw_joints: bool,
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
    pub fn new(scale: f32, pos: Vec2, spider_type: SpiderType) -> Self {
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
        }

        Self {
            pos,
            velocity: Vec2::ZERO,
            spider_type,

            scale,

            face_dir,
            legs,

            max_leg_angle: 0.6,

            debug_leg_angles: false,
            debug_color_legs: false,
            debug_draw_joints: false,
        }
    }

    /// Returns a rotation transform in the direction the spider is facing.
    pub fn face_transform(&self) -> Mat3 {
        Mat3::from_rotation_z(Vec2::new(0.0, 1.0).angle_between(self.face_dir))
    }

    pub fn run_away_from(&mut self, enemy: Vec2) {
        let mut perp_vec = (self.pos - enemy).perp().normalize();

        if self.spider_type == SpiderType::Left {
            perp_vec = perp_vec.perp().perp().normalize();
            // root_ui().label(self.pos, &format!("perp {:#.2?}", self));
        }

        if unsafe { DEBUG_AI_LABELS } {
            let below = enemy.y > self.pos.y;
            let left = enemy.x < self.pos.x;

            root_ui().label(self.pos, &format!("below {} left {}", below, left));
        }

        //         if below && left {
        //             perp_vec = perp_vec.perp().perp();
        //
        //         }

        const LIMIT: f32 = 100.0;

        if self.pos.x <= LIMIT {
            perp_vec.x = 1.0;
        }
        if self.pos.x >= (screen_width() - LIMIT) {
            perp_vec.x = -1.0;
        }

        if self.pos.y <= LIMIT {
            perp_vec.y = 1.0;
        }
        if self.pos.y >= (screen_height() - LIMIT) {
            perp_vec.y = -1.0;
        }

        self.move_towards(perp_vec);
    }

    pub fn move_towards(&mut self, move_dir: Vec2) {
        if move_dir.length() > 0.1 {
            self.velocity += move_dir.normalize() * MOVE_SPEED * get_frame_time();
        }

        self.velocity = self.velocity.clamp_length_max(5.0);

        self.pos += self.velocity;
        self.velocity *= 0.90;

        if self.velocity.length() > 0.01 {
            self.face_dir = self.face_dir.lerp(self.velocity.normalize(), 0.5);
            // Gets rid of NaN when face_dir == [0, 0]
            if self.face_dir.length() < 0.01 {
                self.face_dir = Vec2::new(1.0, 0.0);
            }
        }

        let face_transform = self.face_transform();

        for (i, leg) in self.legs.iter_mut().enumerate() {
            let leg_dir = leg.end - self.pos;
            // let ideal_leg_dir = leg_origin_dir(self.face_dir, i).normalize();
            let ideal_leg_dir = face_transform.transform_vector2(leg.ideal_leg_dir);

            if leg_dir.length() > 2.0 * LEG_LENGTH {
                leg.target = self.pos + ideal_leg_dir * LEG_LENGTH * 1.4;
            }

            if leg_dir.length() < 1.2 * LEG_LENGTH {
                leg.target = self.pos + ideal_leg_dir * LEG_LENGTH * 2.0;
            }

            let angle = leg_dir.angle_between(ideal_leg_dir).abs();

            if self.debug_leg_angles {
                root_ui().label(leg.end, &format!("angle {} = {:.2}", i, angle));
            }

            if angle > self.max_leg_angle {
                leg.end = self.pos + ideal_leg_dir * LEG_LENGTH * 1.6;
            }

            let target = (leg.target - self.pos).clamp_length(16.0, LEG_LENGTH * 2.0);

            let mut mid = target / 2.0;
            let norm = mid.perp();

            let d = |origin: Vec2, a: Vec2, b: Vec2| {
                let d1 = (origin - a).length() - LEG_LENGTH;
                let d2 = (b - a).length() - LEG_LENGTH;

                f32::abs(d1 + d2)
            };

            let mut min_dist = d(leg.origin_offset, mid, target);
            let mut min_mid = mid;

            for iter in 0..1000 {
                // TODO: use leg count
                let sign = if i < 4 { -1.0 } else { 1.0 };

                mid += sign * norm.normalize() * 0.1 * iter as f32;

                let new_dist = d(leg.origin_offset, mid, target);

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
        let body_color = if self.spider_type == SpiderType::Player { PLAYER_BODY_COLOR } else { BODY_COLOR };

        draw_circle(self.pos.x, self.pos.y, R * self.scale, body_color);
        draw_circle(
            self.pos.x - self.face_dir.x * BUTT_OFFSET * self.scale,
            self.pos.y - self.face_dir.y * BUTT_OFFSET * self.scale,
            BUTT_RADIUS * self.scale,
            body_color,
        );

        let colors = [YELLOW, ORANGE, RED, PURPLE, BLUE, GRAY, DARKGRAY, BLACK];

        for (i, leg) in self.legs.iter_mut().enumerate() {
            let lerp_speed = 0.5;
            leg.lerp_mid = leg.lerp_mid.lerp(leg.mid, lerp_speed);
            leg.lerp_end = leg.lerp_end.lerp(leg.end, lerp_speed);

            let color = if self.debug_color_legs {
                colors[i]
            } else {
                body_color
            };

            let lerp_mid_vec = (leg.lerp_mid - self.pos) * self.scale;
            let lerp_end_vec = (leg.lerp_end - self.pos) * self.scale;

            let lerp_mid = self.pos + lerp_mid_vec;
            let lerp_end = self.pos + lerp_end_vec;

            let w = T * self.scale;

            line(self.pos + leg.origin_offset, lerp_mid, w, color);
            line(lerp_mid, lerp_end, w, color);

            let (c1, c2) = if self.debug_draw_joints {
                (GREEN, BLUE)
            } else {
                (body_color, body_color)
            };

            draw_circle(lerp_mid.x, lerp_mid.y, 4.0 * self.scale, c1);
            draw_circle(lerp_end.x, lerp_end.y, 4.0 * self.scale, c2);
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
