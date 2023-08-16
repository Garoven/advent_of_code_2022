mod day_1;
mod day_2;
mod day_3;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args: Args = clap::Parser::parse();
    let input = std::fs::read_to_string(args.input).unwrap();

    match args.day {
        1 => day_1::combined(&input),
        2 => day_2::combined(&input),
        3 => day_3::combined(&input),
        4..=25 => unimplemented!(),
        _ => panic!("Select day between 1 and 25"),
    }
}
