mod cmd;
use anyhow::Result;
use clap::{Parser, Subcommand};

fn main() -> Result<()> {
    let args = CommandArgs::parse();
    match args.subcommand {
        Subcommands::Configure => cmd::configure(&args.profile),
        Subcommands::Login { role_name } => cmd::login(&args.profile, role_name),
    }
}

#[derive(Debug, Parser)]
#[command[version, about, author]]
struct CommandArgs {
    #[command(subcommand)]
    subcommand: Subcommands,
    /// AWS profile
    #[arg(short, long, default_value = "default")]
    profile: String,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    /// Configure the initial settings
    Configure,
    /// Login with SAML SSO, then assume an AWS IAM role
    Login {
        /// AWS IAM role name
        #[arg(short, long)]
        role_name: Option<String>,
    },
}
