// TODO implement this properly

pub struct Camera {
    //
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
