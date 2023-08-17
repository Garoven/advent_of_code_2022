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
        1 => advent_of_code_2022::day_1::combined(&input),
        2 => advent_of_code_2022::day_2::combined(&input),
        3 => advent_of_code_2022::day_3::combined(&input),
        4 => advent_of_code_2022::day_4::combined(&input),
        5 => advent_of_code_2022::day_5::combined(&input),
        6..=25 => unimplemented!(),
        _ => panic!("Select day between 1 and 25"),
    }
}
