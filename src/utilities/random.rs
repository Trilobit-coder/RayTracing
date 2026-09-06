/// Generates a random float in `[0, 1)`.
pub fn random_f32() -> f32 {
    fastrand::f32()
}

/// Generates a random float in `[min, max)`.
pub fn random_f32_range(min: f32, max: f32) -> f32 {
    min + (max - min) * fastrand::f32()
}

/// Generates a random integer in range `[min, max]`.
pub fn random_i32_range(min: i32, max: i32) -> i32 {
    fastrand::i32(min..=max)
}
