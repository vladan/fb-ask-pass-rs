use std::env;

pub fn parse_args() -> Result<Option<String>, &'static str> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            if &args[1] == "--write" { Ok(Some(args[2].clone())) }
            else { Err("only allowed 1st argument is --write") }
        },
        1 => Ok(None),
        _ => Err("only 0 or 2 arguments are allowed")
    }
}
