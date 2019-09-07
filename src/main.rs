mod cli;
mod drawing;
mod passwd;

use framebuffer::{Framebuffer, KdMode};
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let config = cli::get_config().unwrap();

    match config.image_path {
        None => drawing::draw_bgrt(config.device),
        Some(image_path) => drawing::draw_image_centered(config.device, image_path)
    }


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
