extern crate bmp;
extern crate framebuffer;

mod drawing;
mod passwd;

use drawing::Msg;
use passwd::*;
use std::env;
use std::io;
use std::sync::mpsc::Sender;

fn parse_args(args: &[String]) -> Result<Option<String>, &'static str> {
    match args.len() {
        3 => {
            if &args[1] == "--write" {
                Ok(Some(args[2].to_string()))
            } else {
                Err("only allowed 1st argument is --write")
            }
        }
        1 => Ok(None),
        _ => Err("only 0 or 2 arguments are allowed"),
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let write_to = parse_args(&args).unwrap();

    // A function that takes a Sender and returns a function that receives a Msg and sends it
    // through the sender obtained in the first function. An acrobatic move to make it possible to
    // construct the keypress callback that is receives a Key instead of Msg.
    let send_to_draw = |sink: Sender<Msg>| {
        let tx = sink.clone();
        move |msg: Msg| tx.send(msg).unwrap()
    };
    // The draw function takes a Msg and sends it to the drawing thread.
    let draw = send_to_draw(drawing::init()?);
    // This draw_key_callback function takes a Key and wraps it in a Msg before sending it to the
    // drawing thread, hence the closure in send_to_draw.
    let draw_key_callback = |k: Key| draw(Msg::KeyPressed(k));

    // Start graphical mode.
    draw(Msg::Start);

    read_pass(draw_key_callback)
        .and_then(validate_pass)
        .map_err(|_| draw(Msg::Fail))
        .and_then(|pass| {
            draw(Msg::Success);
            write_pass(write_to, pass);
            draw(Msg::Stop);
            Ok(())
        })
        .map_err(|_| {
            draw(Msg::Fail);
            io::Error::new(io::ErrorKind::InvalidData, "FAAIIILLL")
        })
}
