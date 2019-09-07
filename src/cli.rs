extern crate clap;
use clap::{App, Arg};

pub struct Config {
    pub image_path: Option<String>,
    pub pass_path: Option<String>,
    pub device: String,
}

pub fn get_config() -> Result<Config, &'static str> {
    let matches = App::new("FrameBuffer AskPass")
        .arg(
            Arg::with_name("write")
                .short("w")
                .long("write")
                .value_name("FB_WRITE_TO")
                .help("Filename to write the password to. Defaults to showing the password on screen.")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("FB_IMAGE")
                .help("The displayed image filename. Defaults to showing the BGRT image.")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("device")
                .short("d")
                .long("device")
                .value_name("FB_DEVICE")
                .help("The framebuffer device.")
                .required(false)
                .default_value("/dev/fb0")
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        image_path: matches.value_of("image").map(String::from),
        pass_path: matches.value_of("write").map(String::from),
        device: matches.value_of("device").map(String::from).unwrap(),
    })
}
