use clap::{Clap, crate_version};

#[derive(Clap)]
#[clap(version = crate_version!(), author = "moisutsu <moisutsu@gmail.com>")]
pub struct Opts {
    pub commands: Vec<String>,
    #[clap(short, long)]
    pub nice: Option<i32>,
    #[clap(short, long, default_value = "/dev/null")]
    pub log_file: String,
}
