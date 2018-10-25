use std::env;
use std::io;

pub enum Action {
    Test,
    Systemd,
    WriteToFile(String),
    None,
}

pub struct Config {
    pub action: Action,
    logo_image: Option<String>,
    logo_x_offset: u32,
    logo_y_offset: u32,
}

pub fn parse_args() -> io::Result<Config> {
    let mut args = env::args();
    let _command = args.next();

    let action = args.next().map(|s| s); //.as_str().clone());
    let action = match action {
        Some("--test") => Action::Test,
        Some("--systemd") => Action::Systemd,
        Some("--write") => {
            let fname = args.next();
            if let Some(fname) = fname {
                Action::WriteToFile(fname)
            } else {
                Action::None
            }
        },
        _ => Action::None
    };


    Err(io::Error::new(io::ErrorKind::Other, "--write requires a filename"))
}

fn parse_options(args: env::Args) -> io::Result<()> {
    Ok(())
}
