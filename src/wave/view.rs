// TODO implement this properly

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Camera {
    pub fn get_matrix(&self) -> [f32; 16] {
        [
            1.0, 0.0, 0.0, 0.0, // r1
            0.0, 1.0, 0.0, 0.0, // r2
            0.0, 0.0, 1.0, 0.0, // r3
            0.0, 0.0, 0.0, 1.0, // r4
        ]
    }
}
