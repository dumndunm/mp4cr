use super::logger;
use std::{env::args, process};

pub struct Args {
    pub source: String,
    pub output: String,
    pub qual: Option<String>,
}

pub fn collect() -> Args {
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    if args.len() < 2 {
        logger::error("Not enough arguments, you need to provide arguments `input` and `output`");
        process::exit(1);
    }

    let source = String::from(&args[0]);
    let output = String::from(&args[1]);

    Args {
        source,
        output,
        qual: args.get(2).cloned(),
    }
}
