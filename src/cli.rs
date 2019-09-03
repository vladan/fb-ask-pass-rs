extern crate clap;
use clap::{App, Arg};

pub struct Config {
    pub image_path: String,
    pub pass_path: Option<String>,
    pub device: String,
    pub load_bgrt: bool,
}

pub fn get_config() -> Result<Config, &'static str> {
    let matches = App::new("FrameBuffer AskPass")
        .arg(
            Arg::with_name("write")
                .short("w")
                .long("write")
                .value_name("FB_WRITE_TO")
                .help("Path to the file the password is stored in. If not provided the password will be printed on screen.")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("FB_IMAGE")
                .help("The displayed image path.")
                .default_value("/sys/firmware/acpi/bgrt/image")
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
        image_path: matches.value_of("image").map(String::from).unwrap(),
        pass_path: matches.value_of("write").map(String::from),
        device: matches.value_of("device").map(String::from).unwrap(),
        load_bgrt: (matches.occurrences_of("image") == 0),
    })
}
