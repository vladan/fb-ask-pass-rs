extern crate bmp;
extern crate framebuffer;

mod passwd;

use framebuffer::{Framebuffer, KdMode};
use std::env;
use std::io::{self, Read, Write};
use std::fs::{File};


struct Coordinate(u32, u32);
struct Color(u8, u8, u8);
struct Pixel(Coordinate, Color);

struct Frame {
    buffer: Vec<u8>,
    xoffset: u32,
    yoffset: u32,
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
}

struct Shape {
    pixels: Vec<Pixel>,
}

struct Rect {
    min: Coordinate,
    max: Coordinate,
    color: Color,
}

impl Into<Shape> for Rect {
    fn into(&self) -> Shape {
        for i in self.min[0]..self.max[0] {
            for j in self.min[1]..self.max[1] {
                // add the pixels
            }
        }
    }
}

impl Frame {
    fn new(xoffset: u32, yoffset: u32, width: u32, height: u32, bytes_per_pixel: u32) -> Self {
        Self {
            buffer: vec![0u8; (width * height) as usize],
            xoffset,
            yoffset,
            width,
            height,
            bytes_per_pixel,
        }
    }

    fn draw_image(&mut self, path: &str) {
        let img = bmp::open(path).unwrap();
        for (x, y) in img.coordinates() {
            let px = img.get_pixel(x, y);
            let idx = (((y + self.yoffset) * self.width) + ((x + self.xoffset) * self.bytes_per_pixel)) as usize;
            self.buffer[idx] = px.b;
            self.buffer[idx + 1] = px.g;
            self.buffer[idx + 2] = px.r;
        }
    }

    fn draw(&mut self, shape: Shape) {
        for Pixel((x,y), (r, g, b)) in shape.pixels {
            let idx = (((y + self.yoffset) * self.width) + ((x + self.xoffset) * self.bytes_per_pixel)) as usize;
            self.buffer[idx] = b;
            self.buffer[idx + 1] = g;
            self.buffer[idx + 2] = r;
        }
    }
}


fn read_u32_from_file(fname: &str) -> io::Result<u32> {
    let mut f = File::open(fname)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    buffer.trim().parse::<u32>()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "can't parse number"))
}


fn parse_args(args: &[String]) -> Result<Option<String>, &'static str> {
    match args.len() {
        3 => {
            if &args[1] == "--write" { Ok(Some(args[2].clone())) }
            else { Err("only allowed 1st argument is --write") }
        },
        1 => Ok(None),
        _ => Err("only 0 or 2 arguments are allowed")
    }
}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let write_to = parse_args(&args).unwrap();

    // Disable text mode in current tty
    let _ = Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();

    let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();

    let xoffset = read_u32_from_file("/sys/firmware/acpi/bgrt/xoffset")?;
    let yoffset = read_u32_from_file("/sys/firmware/acpi/bgrt/yoffset")?;
    let width = framebuffer.fix_screen_info.line_length;
    let height = framebuffer.var_screen_info.yres;
    let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;

    let mut frame = Frame::new(xoffset, yoffset, width, height, bytespp);
    frame.draw_image("/sys/firmware/acpi/bgrt/image");

    let _ = framebuffer.write_frame(frame.buffer.as_slice());

    let feedback = || { };
    let pass = passwd::read_pass(&feedback)?;

    match write_to {
        None => {
            // for testing, get back to text mode
            let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
            println!("You entered: {}", pass);
        },
        Some(fname) => {
            let mut f = File::create(fname)?;
            f.write(pass.as_bytes())?;
        }
    }

    Ok(())
}
