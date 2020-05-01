use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Camera {
    pub fn get_matrix(&self) -> cgmath::Matrix4<f32> {
        let mat: Matrix4<f32> = Matrix4::identity();
        let mat = mat
            * Matrix4::from_axis_angle(
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Rad(self.pitch),
            );
        let mat = mat
            * Matrix4::from_axis_angle(
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Rad(self.yaw),
            );
        let mat = mat
            * Matrix4::from_axis_angle(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Rad(self.roll),
            );
        let mat = mat
            * Matrix4::from_translation(Vector3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            });

        mat
    }
}
