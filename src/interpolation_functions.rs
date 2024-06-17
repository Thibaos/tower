#![allow(unused)]

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a).mul_add(t, a)
}

pub fn ease_out_expo(x: f32) -> f32 {
    1.0 - f32::powf(2.0, -10.0 * x)
}
