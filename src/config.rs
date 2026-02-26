use clap::Parser;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct Config {
    #[clap(long, env)]
    pub migration_path: String,
    #[clap(long, env)]
    pub database_url: String,
}
