use crate::wave::bundles::resource::ResourceBundle;
use crate::wave::bundles::window::WindowBundle;
use cull_canyon::{MTLBuffer, MTLTextureDescriptor};
use std::collections::HashMap;
use std::os::raw::c_void;

pub struct Letter {
    pub buffer: MTLBuffer,
    pub texture_coords: (f32, f32),
    pub x_advance: u32,
}

pub struct TerminalBundle {
    pub letter_map: HashMap<char, Letter>,
}

fn read_font_file(
    contents: &str,
    bundle: &WindowBundle,
    resource_bundle: &ResourceBundle,
) -> HashMap<char, Letter> {
    let mut letter_map = HashMap::new();

    contents.split("\r\n").for_each(|s: &str| {
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
        let id: char = unsafe { std::mem::transmute(id) };
        let x = x;
        let y = y;
        let width = width as u32;
        let height = height as u32;
        let x_offset = x_offset;
        let y_offset = y_offset;
        let x_advance = x_advance as u32;

        // letter_map.insert(
        //     id,
        //     Letter {
        //         buffer: placeholder,
        //         texture_coords: (0.0, 0.0),
        //         x_advance,
        //     },
        // );
    });

    letter_map
}

impl TerminalBundle {
    pub fn new(bundle: &WindowBundle, resource_bundle: &ResourceBundle) -> TerminalBundle {
        let letter_map = read_font_file(
            std::fs::read_to_string("resources/tahoma.fnt")
                .unwrap()
                .as_str(),
            bundle,
            resource_bundle,
        );

        let decoder = png::Decoder::new(std::fs::File::open("resources/tahoma.png").unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut img = vec![0; info.buffer_size()];
        reader.next_frame(&mut img).unwrap();

        let atlas_texture = unsafe {
            resource_bundle.device.new_texture_with_descriptor({
                let desc = MTLTextureDescriptor::new();
                desc.set_width(info.width as u64);
                desc.set_height(info.height as u64);
                desc.set_pixel_format(70); // rgba8unorm
                desc
            })
        };

        unsafe {
            atlas_texture.replace_region(
                (0, 0, info.width as u64, info.height as u64),
                0,
                img.as_mut_ptr() as *mut c_void,
                4 * info.width as u64, // 4 because rgba8unorm is 4 bytes per pixel
            );
        };

        TerminalBundle { letter_map }
    }
}
