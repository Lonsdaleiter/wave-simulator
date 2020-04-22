use cull_canyon::MTLBuffer;
use crate::wave::bundles::window::WindowBundle;
use std::collections::HashMap;

pub struct Letter {
    pub buffer: MTLBuffer,
    pub texture_coords: (f32, f32),
    pub x_advance: u32,
    pub x_offset: i32,
    pub y_offset: i32,
}

pub struct TerminalBundle {
    pub letter_map: HashMap<char, Letter>,
}

fn read_font_file(contents: &str, bundle: &WindowBundle) -> HashMap<char, Letter> {
    let mut letter_map = HashMap::new();

    contents.split("\r\n").for_each(|s: &str|{
        let mut id = 0;
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;
        let mut x_offset = 0;
        let mut y_offset = 0;
        let mut x_advance = 0;
        s.split(" ").for_each(|s: &str| {
            if !s.eq("") && !s.eq("char") {
                let k = s.split("=").collect::<Vec<_>>();
                let item = k[0];
                let val = k[1];
                let value: &mut i32 = match item {
                    "id" => &mut id,
                    "x" => &mut x,
                    "y" => &mut y,
                    "width" => &mut width,
                    "height" => &mut height,
                    "xoffset" => &mut x_offset,
                    "yoffset" => &mut y_offset,
                    "xadvance" => &mut x_advance,
                    _ => return,
                };
                *value = val.parse().unwrap();
            }
        });
        println!("id: {}", id);
        println!("x: {}", x);
        println!("y: {}", y);
        println!("width: {}", width);
        println!("height: {}", height);
        println!("x offset: {}", x_offset);
        println!("y offset: {}", y_offset);
        println!("x advance: {}", x_advance);
    });

    letter_map
}

impl TerminalBundle {
    pub fn new(bundle: &WindowBundle) -> TerminalBundle {
        let letter_map = read_font_file(
            std::fs::read_to_string("resources/tahoma.fnt")
                .unwrap()
                .as_str(),
            bundle,
        );

        TerminalBundle { letter_map }
    }
}
