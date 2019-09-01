mod drawing;
mod passwd;
mod cli;

use crate::drawing::Frame;
use framebuffer::{Framebuffer, KdMode};
use std::env;
use std::fs::File;
use std::io::{self, Write};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = cli::parse_args(&args).unwrap();

    let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();
    let frame = match config.image_path {
        None => {
            drawing::frame_from_bgrt(&framebuffer)
        }
        Some(img_fname) => {
            let mut frame = Frame::new(&framebuffer);
            frame.draw_image(&img_fname);
            frame
        }
    };
    framebuffer.write_frame(frame.buffer.as_slice());

    let feedback = || {};
    let pass = passwd::read_pass(&feedback)?;

    match config.pass_path {
        None => {
            // for testing, get back to text mode
            let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
            println!("You entered: {}", pass);
        }
        Some(fname) => {
            let mut f = File::create(fname)?;
            f.write_all(pass.as_bytes())?;
        }
    }

    Ok(())
}
