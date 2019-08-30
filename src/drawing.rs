extern crate framebuffer;

use framebuffer::Framebuffer;
use std::fs::File;
use std::io::{self, Read};

pub struct Frame {
    pub buffer: Vec<u8>,
    xoffset: u32,
    yoffset: u32,
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
}

impl Frame {
    pub fn new(fb: &Framebuffer) -> Self {
        let xoffset = read_u32_from_file("/sys/firmware/acpi/bgrt/xoffset").unwrap();
        let yoffset = read_u32_from_file("/sys/firmware/acpi/bgrt/yoffset").unwrap();
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

    pub fn draw_image(&mut self, path: String) {
        let img = bmp::open(path.as_ref()).unwrap();
        for (x, y) in img.coordinates() {
            let px = img.get_pixel(x, y);
            let idx = (((y + self.yoffset) * self.width)
                + ((x + self.xoffset) * self.bytes_per_pixel)) as usize;
            self.buffer[idx] = px.b;
            self.buffer[idx + 1] = px.g;
            self.buffer[idx + 2] = px.r;
        }
    }
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