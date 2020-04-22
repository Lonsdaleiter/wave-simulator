use cull_canyon::MTLBuffer;

pub type Letters = [Letter; 128];

pub struct Letter {
    pub buffer: MTLBuffer,
    pub texture_coords: (f32, f32),
    pub x_advance: u32,
    pub x_offset: i32,
    pub y_offset: i32,
}

pub struct TerminalBundle {
    //
}
