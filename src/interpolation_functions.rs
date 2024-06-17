#![allow(unused)]

use crate::entity::levels::PI;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a).mul_add(t, a)
}

pub fn ease_out_expo(x: f32) -> f32 {
    1.0 - f32::powf(2.0, -10.0 * x)
}

pub fn ease_out_cubic(x: f32) -> f32 {
    1.0 - f32::powf(1.0 - x, 3.0)
}

pub fn ease_out_circ(x: f32) -> f32 {
    f32::sqrt(1.0 - f32::powi(x - 1.0, 2))
}

pub fn ease_out_sine(x: f32) -> f32 {
    f32::sin((x * PI) / 2.0)
}
