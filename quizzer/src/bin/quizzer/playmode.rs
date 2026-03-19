use clap::Subcommand;

#[derive(Subcommand)]
pub(crate) enum PlayMode {
    Question,
    Quiz,
}