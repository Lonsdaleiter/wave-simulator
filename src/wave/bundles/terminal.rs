use crate::wave::bundles::resource::ResourceBundle;
use crate::wave::bundles::window::WindowBundle;
use cull_canyon::{
    MTLBuffer, MTLSamplerDescriptor, MTLSamplerState, MTLTexture, MTLTextureDescriptor,
};
use std::collections::HashMap;
use std::os::raw::c_void;

pub struct Letter {
    pub buffer: MTLBuffer,
    pub x_offset: i32,
    pub y_offset: i32,
    pub x_advance: u32,
    pub width: f32,
    pub height: f32,
}

pub struct TextBundle {
    pub letter_map: HashMap<char, Letter>,
    pub atlas_texture: MTLTexture,
    pub sampler: MTLSamplerState,
    pub current_text: Vec<char>,
}

fn read_font_file(
    contents: &str,
    bundle: &WindowBundle,
    resource_bundle: &ResourceBundle,
    texture_size: (u32, u32),
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

        let real_x: f32 = x as f32 / texture_size.0 as f32;
        let real_y: f32 = y as f32 / texture_size.1 as f32;
        let real_width: f32 = width as f32 / texture_size.0 as f32;
        let real_height: f32 = height as f32 / texture_size.1 as f32;

        // 6 vertices + 6 texture coords
        // 4 floats each
        // 4 bytes per float
        // size is 192
        let base_data = [
            // triangle 1
            -1.0f32 * real_width,
            -1.0 * real_height, // v1
            0.0 + real_x,
            1.0 * real_height + real_y, // t1
            -1.0 * real_width,
            1.0 * real_height, // v2
            0.0 + real_x,
            0.0 + real_y, // t2
            1.0 * real_width,
            1.0 * real_height, // v3
            1.0 * real_width + real_x,
            0.0 + real_y, // t3
            // triangle 2
            1.0 * real_width,
            1.0 * real_height, // v3
            1.0 * real_width + real_x,
            0.0 + real_y, // t3
            1.0 * real_width,
            -1.0 * real_height, // v4
            1.0 * real_width + real_x,
            1.0 * real_height + real_y, // t4
            -1.0f32 * real_width,
            -1.0 * real_height, // v1
            0.0 + real_x,
            1.0 * real_height + real_y, // t1
        ];
        let buffer = unsafe {
            resource_bundle.device.new_buffer_with_bytes(
                base_data.as_ptr() as *const c_void,
                base_data.len() as u64 * 4,
                0,
            )
        };

        letter_map.insert(
            id,
            Letter {
                // buffer contains both positions and texture coords
                buffer,
                x_offset,
                y_offset,
                x_advance,
                width: real_width,
                height: real_height,
            },
        );
    });

    letter_map
}

impl TextBundle {
    pub fn new(bundle: &WindowBundle, resource_bundle: &ResourceBundle) -> TextBundle {
        let decoder = png::Decoder::new(std::fs::File::open("resources/tahoma.png").unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut img = vec![0; info.buffer_size()];
        reader.next_frame(&mut img).unwrap();

        let letter_map = read_font_file(
            std::fs::read_to_string("resources/tahoma.fnt")
                .unwrap()
                .as_str(),
            bundle,
            resource_bundle,
            (info.width, info.height),
        );

        let atlas_texture = unsafe {
            resource_bundle.device.new_texture_with_descriptor({
                let desc = MTLTextureDescriptor::new();
                desc.set_width(info.width as u64);
                desc.set_height(info.height as u64);
                desc.set_pixel_format(70); // rgba8unorm
                desc
            })
        };

        let sampler = unsafe {
            atlas_texture.replace_region(
                (0, 0, info.width as u64, info.height as u64),
                0,
                img.as_mut_ptr() as *mut c_void,
                4 * info.width as u64, // 4 because rgba8unorm is 4 bytes per pixel
            );

            // let mut k = vec![0; info.buffer_size()];
            // atlas_texture.get_bytes(
            //     k.as_mut_ptr() as *mut c_void,
            //     4 * info.width as u64,
            //     (0, 0, info.width as u64, info.height as u64),
            //     0,
            // );
            // assert_eq!(k, img);

            resource_bundle
                .device
                .new_sampler_state_with_descriptor(MTLSamplerDescriptor::new())
        };

        TextBundle {
            letter_map,
            atlas_texture,
            sampler,
            current_text: "> a test here; guess what? too lazy, it's gone.".chars().collect(),
        }
    }
}
