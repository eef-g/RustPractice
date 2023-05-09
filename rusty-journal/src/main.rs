pub mod cli;
use structopt::StructOpt;
fn main() {
    println!("{:#?}",cli::args::CommandLineArgs::from_args());
    return;
}
