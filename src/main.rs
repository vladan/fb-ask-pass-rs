mod cli;
mod drawing;
mod passwd;

use framebuffer::{Framebuffer, KdMode};
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let config = cli::get_config().unwrap();

    drawing::init(config.device, config.image_path, config.load_bgrt);

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
