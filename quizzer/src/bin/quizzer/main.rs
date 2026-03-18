mod cli;
mod playmode;

fn main() -> anyhow::Result<()> {
    let cli = cli::parse();
    match cli.command {

    }
}