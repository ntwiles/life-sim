pub fn alpha_blend(source: [u8; 4], destination: [u8; 4]) -> [u8; 4] {
    let alpha = source[3] as f32 / 255.0;
    let inv_alpha = 1.0 - alpha;

    let r = (source[0] as f32 * alpha + destination[0] as f32 * inv_alpha) as u8;
    let g = (source[1] as f32 * alpha + destination[1] as f32 * inv_alpha) as u8;
    let b = (source[2] as f32 * alpha + destination[2] as f32 * inv_alpha) as u8;
    let a = 255;

    [r, g, b, a]
}

pub fn additive_blend(source: [u8; 4], dest: [u8; 4]) -> [u8; 4] {
    let r = (source[0] as f32 + dest[0] as f32).min(255.0) as u8;
    let b = (source[1] as f32 + dest[1] as f32).min(255.0) as u8;
    let g = (source[2] as f32 + dest[2] as f32).min(255.0) as u8;
    let a = 255;

    [r, b, g, a]
}
