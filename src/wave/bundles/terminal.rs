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
        s.split(" ").for_each(|s: &str| {
            if !s.eq("") && !s.eq("char") {
                println!("{}", s);
            }
        });
        println!();
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
