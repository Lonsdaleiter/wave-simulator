// TODO implement this properly

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

impl Camera {
    pub fn get_matrix(&self) -> [f32; 16] {
        let cos_pitch = self.pitch.cos();
        let sin_pitch = self.pitch.sin();
        let cos_yaw = self.yaw.cos();
        let sin_yaw = self.yaw.sin();
        let cos_roll = self.roll.cos();
        let sin_roll = self.roll.sin();
        [
            cos_yaw, sin_yaw * sin_pitch, sin_yaw * cos_pitch, 0.0, // r1
            0.0, cos_pitch, -sin_pitch, 0.0, // r2
            -sin_yaw, cos_yaw * sin_pitch, cos_pitch * cos_yaw, 0.0, // r3
            0.0, 0.0, 0.0, 1.0,
            // -dot(), -dot(), -dot(), 1.0, // r4 TODO finish up the dots
        ]
    }
}
