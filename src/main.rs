use structopt::StructOpt;
mod day01;
mod day02;
// #[macro_use]
// extern crate lazy_static;
// #[macro_use]
// extern crate maplit;

#[derive(StructOpt)]
struct Cli {
    day: u8,
}

fn main() {
    let args = Cli::from_args();
    match args.day {
        1 => day01::day01(),
        2 => day02::day02(),
        _ => println!("Unimplemented day: {}", args.day),
    }
}