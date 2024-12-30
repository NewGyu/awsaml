use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = CommandArgs::parse();
    println!("{:?}", args);
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
