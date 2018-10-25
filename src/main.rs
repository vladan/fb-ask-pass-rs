extern crate bmp;
extern crate framebuffer;

mod passwd;
mod cli;

use framebuffer::{Framebuffer, KdMode};
use std::io::{self, Read, Write};
use std::fs::{File};


fn read_u32_from_file(fname: &str) -> io::Result<u32> {
    let mut f = File::open(fname)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    buffer.trim().parse::<u32>()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "can't parse number"))
}


fn main() -> io::Result<()> {

    let config = cli::parse_args().unwrap();

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

    let feedback = || { };
    let pass = passwd::read_pass(&feedback)?;

    match config.action {
        cli::Action::Test => {
            // for testing, get back to text mode
            let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
            println!("You entered: {}", pass);
        },
        cli::Action::WriteToFile(fname) => {
            let mut f = File::create(fname)?;
            f.write(pass.as_bytes())?;
        },
        _ => ()
    }

    Ok(())
}
