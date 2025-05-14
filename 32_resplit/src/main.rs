use clap::Parser;
use cli_lib::read_stdin;
use resplit::Cli;

fn main() {
    let cli = Cli::parse();
    // let buffer = resplit::read_stdin();
    let buffer = read_stdin();

    let result = resplit::split(buffer, &cli);
    println!("{}", result);
}
