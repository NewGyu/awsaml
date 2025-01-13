mod cmd;
use crate::cmd::config::Config;
use anyhow::Result;
use clap::{Parser, Subcommand};

fn main() -> Result<()> {
    let args = CommandArgs::parse();
    println!("{:?}", args);
    match args.subcommand {
        Subcommands::Configure => {
            let new_config = Config::configure()?;
            new_config.save(&args.profile)?;
        }
        Subcommands::Login { role_name } => {
            let config = Config::load(&args.profile)?;
            println!("{:?}", config);
        }
    }

    Ok(())
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
