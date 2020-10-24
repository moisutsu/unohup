use clap::Clap;
use std::process::Command;
use unohup::Opts;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let commands = format!(
        "nice -n {} nohup {} > {} 2>&1 &",
        opts.nice,
        opts.commands.join(" "),
        opts.log_file
    );

    Command::new("bash").arg("-c").arg(&commands).spawn()?;

    Ok(())
}
