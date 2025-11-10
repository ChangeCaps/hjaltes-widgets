use clap::Parser;

mod bar;
mod notify;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    cmd: Subcommand,
}

#[derive(Parser)]
enum Subcommand {
    Bar,
    Notify,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        Subcommand::Bar => bar::run(),
        Subcommand::Notify => notify::run(),
    }
}
