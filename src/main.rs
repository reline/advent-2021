mod cli;
mod day7;
mod io;

fn main() {
    let args = cli::parse_args();
    match args.day.as_str() {
        "7" => {
            let input = io::split_num_commas(args.path);
            println!("Part One: {}", day7::partone(&input))
        }
        _ => {
            println!("Incomplete day")
        }
    }
}
