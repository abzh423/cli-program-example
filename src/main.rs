extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// string object that will be printed out
    message: String,
    #[structopt(short = "d", long = "dead")]
    /// make the cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    println!("{}", message);
}
