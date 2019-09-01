extern crate framebuffer;

use framebuffer::Framebuffer;
use std::fs::File;
use std::io::{self, Read};

pub struct Frame {
    pub buffer: Vec<u8>,
    xoffset: Option<u32>,
    yoffset: Option<u32>,
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
}

impl Frame {
    pub fn new(fb: &Framebuffer) -> Self {
        let xoffset = None;
        let yoffset = None;
        let width = fb.fix_screen_info.line_length;
        let height = fb.var_screen_info.yres;
        let bytes_per_pixel = fb.var_screen_info.bits_per_pixel / 8;

        Self {
            buffer: vec![0u8; (width * height) as usize],
            xoffset,
            yoffset,
            width,
            height,
            bytes_per_pixel,
        }
    }

    pub fn draw_image(&mut self, path: &str) {
        let img = bmp::open(path).unwrap();
        let xoffset = self
            .xoffset
            .unwrap_or((self.width / self.bytes_per_pixel) / 2 - img.get_width() / 2);
        let yoffset = self
            .yoffset
            .unwrap_or(self.height / 2 - img.get_height() / 2);

        for (x, y) in img.coordinates() {
            let px = img.get_pixel(x, y);
            let xb = (x + xoffset) * self.bytes_per_pixel;
            let yb = (y + yoffset) * self.width;
            let idx = (xb + yb) as usize;
            self.buffer[idx] = px.b;
            self.buffer[idx + 1] = px.g;
            self.buffer[idx + 2] = px.r;
        }
    }
}

pub fn frame_from_bgrt(fb: &Framebuffer) -> Frame {
        let xoffset = read_u32_from_file("/sys/firmware/acpi/bgrt/xoffset").ok();
        let yoffset = read_u32_from_file("/sys/firmware/acpi/bgrt/yoffset").ok();
        let width = fb.fix_screen_info.line_length;
        let height = fb.var_screen_info.yres;
        let bytes_per_pixel = fb.var_screen_info.bits_per_pixel / 8;

        let mut frame = Frame {
            buffer: vec![0u8; (width * height) as usize],
            xoffset,
            yoffset,
            width,
            height,
            bytes_per_pixel,
        };
        frame.draw_image("/sys/firmware/acpi/bgrt/image");
        frame
}

fn read_u32_from_file(fname: &str) -> io::Result<u32> {
    let mut f = File::open(fname)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    buffer
        .trim()
        .parse::<u32>()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "can't parse number"))
}
