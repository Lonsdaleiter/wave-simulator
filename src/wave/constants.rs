use cgmath::Deg;

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;
pub const FPS: f32 = 60.0;
pub const VSYNC: bool = true;
pub const FOV: f32 = 70.0; // degrees
pub const FAR_PLANE: f32 = 1000.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const CAMERA_SPEED: f32 = 0.1;
pub const VERTEX_COUNT: u32 = 100;

pub static mut FILL_MODE: u64 = 1; // 0 = triangles, 1 = lines

pub const FREQ_OF_UPDATES: u64 = 5; // the lower the more frequent
pub const MAX_RAYCAST_DISTANCE: u64 = 100;
pub const RAYCAST_RES: u64 = 100;
pub const RAYCAST_CLOSENESS_REQ: f32 = 1.0;

pub fn new_projection_matrix(aspect_ratio: f32) -> cgmath::Matrix4<f32> {
    let persp = cgmath::perspective(Deg(FOV), aspect_ratio, NEAR_PLANE, FAR_PLANE);
    persp
}
