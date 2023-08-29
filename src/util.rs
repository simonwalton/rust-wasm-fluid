pub fn add_array(a: &mut [f32], b: &[f32]) {
    for i in 0..a.len() {
        a[i] += b[i];
    }
}

#[inline(always)]
pub fn xy(x: u32, y: u32, n: u32) -> usize {
    (x + (y * n)) as usize
}

#[inline(always)]
pub fn clamp(x: f32) -> f32 {
    x.max(0.0f32).min(1.0f32)
}

#[inline(always)]
pub fn clamp_ab(x: f32, a: f32, b: f32) -> f32 {
    x.max(a).min(b)
}

#[inline(always)]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + ((b - a) * t)
}