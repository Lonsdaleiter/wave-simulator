pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;
pub const FPS: f32 = 60.0;
pub const VSYNC: bool = true;
pub const FOV: f32 = 70.0; // degrees
pub const FAR_PLANE: f32 = 1000.0;
pub const NEAR_PLANE: f32 = 0.1;

pub fn new_projection_matrix(aspect_ratio: f32) -> [f32; 16] {
    let y_scale: f32 = (1.0 / (FOV.to_radians() / 2.0)).tan();
    let x_scale = y_scale / aspect_ratio;
    let frustum_len = FAR_PLANE - NEAR_PLANE;
    [
        x_scale, 0.0, 0.0, 0.0,
        0.0, y_scale, 0.0, 0.0,
        0.0, 0.0, -FAR_PLANE / frustum_len, -1.0,
        0.0, 0.0, -FAR_PLANE * NEAR_PLANE / frustum_len, 0.0,
    ]
}
