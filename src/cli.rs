use std::env;
use std::io;

pub fn usage() -> &'static str {
r#"
Usage:

Commands:
  --test            Run in test mode, exit graphics mode and show the entered text on console
  --write <FILE>    Write passphrase to FILE
  --systemd         Run as systemd password agent

Options:
  --fb <DEVICE>     Framebuffer device [default /dev/fb0]
  --image <FILE>    Image to display [default /sys/firmware/acpi/bgrt/image]
  --offset <X Y>       Offset for --image

"#
}

pub enum Action {
    Test,
    Systemd,
    WriteToFile(String),
    None,
}

pub struct Config {
    pub action: Action,
    framebuffer: String,
    logo_image: String,
    logo_offset: (u32, u32),
}

pub fn parse_args() -> io::Result<Config> {
    let mut args = env::args();
    let _command = args.next();


    let action = parse_action(&mut args);
    let options = parse_options(args);
    Err(io::Error::new(io::ErrorKind::Other, usage()))
}

fn parse_action(args: &mut env::Args) -> Action {
    let mut action_opt = args.next();
    let action_str = action_opt.as_ref().map(|s| s.as_str());
    match action_str {
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
        Some(_) => Action::None,
        None => Action::None
    }
}

fn parse_options(args: env::Args) -> io::Result<()> {
    Ok(())
}
