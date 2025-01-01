mod cmd;
use anyhow::Result;
use clap::Parser;
use cmd::config::Config;

fn main() -> Result<()> {
    let args = CommandArgs::parse();
    println!("{:?}", args);
    let config = if args.configure {
        Config::configure(&args.profile)?
    } else {
        Config::load(&args.profile)?
    };
    println!("{:?}", config);

    Ok(())
}

#[derive(Debug, Parser)]
#[command[version, about, author]]
struct CommandArgs {
    /// Configure the initial settings
    #[arg(short, long, default_value = "false")]
    configure: bool,
    /// AWS profile
    #[arg(short, long, default_value = "default")]
    profile: String,
    /// AWS IAM role name
    #[arg(short, long)]
    role_name: Option<String>,
    /// Open AWS management console in the browser
    #[arg(short, long, default_value = "false")]
    web: bool,
}
