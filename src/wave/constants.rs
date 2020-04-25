use cgmath::Deg;

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;
pub const FPS: f32 = 60.0;
pub const VSYNC: bool = true;
pub const FOV: f32 = 70.0; // degrees
pub const FAR_PLANE: f32 = 1000.0;
pub const NEAR_PLANE: f32 = 0.1;

pub fn new_projection_matrix(aspect_ratio: f32) -> [f32; 16] {
    // let y_scale: f32 = (1.0 / (FOV / 2.0).to_radians()).tan();
    // let x_scale = y_scale / aspect_ratio;
    // let frustum_len = FAR_PLANE - NEAR_PLANE;
    unsafe {
        let persp = cgmath::perspective(Deg(FOV), aspect_ratio, NEAR_PLANE, FAR_PLANE);
        std::mem::transmute(persp)
    }
    // [
    //     x_scale,
    //     0.0,
    //     0.0,
    //     0.0, // r1
    //     0.0,
    //     y_scale,
    //     0.0,
    //     0.0, // r2
    //     0.0,
    //     0.0,
    //     -((FAR_PLANE + NEAR_PLANE) / frustum_len),
    //     -1.0, // r3
    //     0.0,
    //     0.0,
    //     -((2.0 * FAR_PLANE * NEAR_PLANE) / frustum_len),
    //     0.0, // r4
    // ]
}
