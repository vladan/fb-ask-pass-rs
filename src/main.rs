extern crate bmp;
extern crate framebuffer;
extern crate rpassword;

use framebuffer::{Framebuffer, KdMode};
use std::env;
use std::io;
use std::io::{Read, Write};
use std::fs::{File};


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

    let img = bmp::open("/sys/firmware/acpi/bgrt/image").unwrap();
    let xoffset = read_u32_from_file("/sys/firmware/acpi/bgrt/xoffset")?;
    let yoffset = read_u32_from_file("/sys/firmware/acpi/bgrt/yoffset")?;

    let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();

    let h = framebuffer.var_screen_info.yres;
    let line_length = framebuffer.fix_screen_info.line_length;
    let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;

    // Disable text mode in current tty
    let _ = Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();
    let mut frame = vec![0u8; (line_length * h) as usize];

    for (x, y) in img.coordinates() {
        let px = img.get_pixel(x, y);
        let start_index = ((y + yoffset) * line_length + (xoffset + x) * bytespp) as usize;
        frame[start_index] = px.b;
        frame[start_index + 1] = px.g;
        frame[start_index + 2] = px.r;
    }

    let _ = framebuffer.write_frame(&frame);

    let pass = rpassword::read_password()?;

    // Don't reenable text mode in current tty, let X handle it afterwards
    // let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();

    match write_to {
        None => println!("You entered: {}", pass),
        Some(fname) => {
            let mut f = File::create(fname)?;
            f.write(pass.as_bytes())?;
        }
    }

    Ok(())
}
