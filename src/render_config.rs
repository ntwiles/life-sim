use colorgrad::Gradient;

pub struct RenderConfig {
    pub pixel_scale: u32,
    pub color_gradient: Gradient,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub killzone_color: [u8; 4],
}
