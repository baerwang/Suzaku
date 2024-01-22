use clap::ArgAction::Set;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, subcommand_precedence_over_arg = true)]
pub struct CLi {
    /// host address
    #[arg(short, long, default_value = ":8080", action = Set)]
    pub address: String,
    /// DEBUG ERROR WARN
    #[arg(short, long, default_value = "INFO", action = Set)]
    pub log_level: String,
}
