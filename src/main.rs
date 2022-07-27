extern crate colored;
extern crate structopt;
use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Read};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// string object that will be printed out
    message: String,
    #[structopt(short = "d", long = "dead")]
    /// make the cat appear dead
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
    #[structopt(short = "i", long = "stdin")]
    /// Read the message from stdin
    stdin: bool,
}

type Handler = Result<(), ExitFailure>;

fn main() -> Handler {
    let options = Options::from_args();
    let mut message = String::new();

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    let eye = if options.dead { "x" } else { "o" };

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog!");
        std::process::exit(1);
    }

    match &options.catfile {
        Some(path) => {
            let template = std::fs::read_to_string(path)
                .with_context(|_| format!("Could not read file {}", path.display()))?;
            let picture = template.replace("{eye}", eye);
            println!("{}", &picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!(" \\");
            println!(" /\\_/\\");
            println!(" ( {eye} {eye} )", eye = eye.red().bold());
            println!(" =( I )=");
        }
    }

    Ok(())
}
