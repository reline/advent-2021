mod cli;

fn main() {
    let args = cli::parse_args();
    match args.day.as_str() {
        "7" => {
            println!("Day 7");
        },
        _ => {
            println!("Incomplete day")
        }
    }
}
