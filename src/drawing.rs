extern crate framebuffer;

use framebuffer::Framebuffer;
use std::fs::File;
use std::io::{self, Read};

pub struct Frame {
    pub buffer: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
}

impl Frame {
    pub fn new(fb: &Framebuffer) -> Self {
        let width = fb.fix_screen_info.line_length;
        let height = fb.var_screen_info.yres;
        let bytes_per_pixel = fb.var_screen_info.bits_per_pixel / 8;

        Self {
            buffer: vec![0u8; (width * height) as usize],
            width,
            height,
            bytes_per_pixel,
        }
    }

    pub fn from_image(fb: &Framebuffer, path: &str, xoffset: Option<u32>, yoffset: Option<u32>) -> Self {
        let mut frame = Self::new(fb);
        let img = bmp::open(path).unwrap();
        let xof = xoffset.unwrap_or((frame.width / frame.bytes_per_pixel) / 2 - img.get_width() / 2);
        let yof = yoffset.unwrap_or(frame.height / 2 - img.get_height() / 2);

        for (x, y) in img.coordinates() {
            let px = img.get_pixel(x, y);
            let xb = (x + xof) * frame.bytes_per_pixel;
            let yb = (y + yof) * frame.width;
            let idx = (xb + yb) as usize;
            frame.buffer[idx] = px.b;
            frame.buffer[idx + 1] = px.g;
            frame.buffer[idx + 2] = px.r;
        }
        frame
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

pub fn draw_image(device: String, image_path: String) {
    let mut framebuffer = Framebuffer::new(device).unwrap();
    let frame = Frame::from_image(&framebuffer, &image_path, None, None);
    framebuffer.write_frame(frame.buffer.as_slice());
}

pub fn draw_bgrt(device: String) {
    let mut framebuffer = Framebuffer::new(device).unwrap();
    let xoffset = read_u32_from_file("/sys/firmware/acpi/bgrt/xoffset").ok();
    let yoffset = read_u32_from_file("/sys/firmware/acpi/bgrt/yoffset").ok();
    let frame = Frame::from_image(&framebuffer,"/sys/firmware/acpi/bgrt/image", xoffset, yoffset);
    framebuffer.write_frame(frame.buffer.as_slice());
}
