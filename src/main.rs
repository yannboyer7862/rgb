use std::{env, process};
use rgb::logging::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        logger_log(LogType::Error, "No ROM provided !");
        logger_log(LogType::Info, "Usage : ./rgb my_game.gb");
        process::exit(1);
    }

    let rom_path = &args[1];
    let bootrom_path = &args[2];
}
