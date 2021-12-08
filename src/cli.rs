use std::path::PathBuf;
use structopt::StructOpt;

pub fn parse_args() -> Args {
    Args::from_args()
}

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short, long)]
    pub day: String,
    #[structopt(short, long, parse(from_os_str))]
    pub path: PathBuf,
}