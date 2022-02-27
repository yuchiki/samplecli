use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Yuchiki",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulae written in RPN
    #[clap(name = "FILE")]
    formulae_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    match opts.formulae_file {
        Some(file) => println!("File speficied: {}", file),
        None => println!("No file specified."),
    }

    println!("Is verbosity specified?: {}", opts.verbose);
}
