use cgmath::Deg;

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;
pub const FPS: f32 = 60.0;
pub const VSYNC: bool = true;
pub const FOV: f32 = 70.0; // degrees
pub const FAR_PLANE: f32 = 1000.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const CAMERA_SPEED: f32 = 0.1;

pub fn new_projection_matrix(aspect_ratio: f32) -> [f32; 16] {
    let persp = cgmath::perspective(Deg(FOV), aspect_ratio, NEAR_PLANE, FAR_PLANE);
    unsafe { std::mem::transmute(persp) }
}
