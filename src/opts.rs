use clap::{crate_version, Clap};

#[derive(Clap)]
#[clap(version = crate_version!(), author = "moisutsu <moisutsu@gmail.com>")]
pub struct Opts {
    pub commands: Vec<String>,
    #[clap(short, long, default_value = "0")]
    pub nice: i32,
    #[clap(short, long, default_value = "/dev/null")]
    pub log_file: String,
}
