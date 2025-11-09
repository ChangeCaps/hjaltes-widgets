use clap::Parser;

mod notify;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    cmd: Subcommand,
}

#[derive(Parser)]
enum Subcommand {
    Notify,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        Subcommand::Notify => notify::run(),
    }
}
