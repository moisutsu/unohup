use unohup::Opts;
use clap::Clap;

fn main() {
    let opts = Opts::parse();

    println!("{:?} {:?} {:?}", opts.commands, opts.nice, opts.log_file);
}
