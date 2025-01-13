use anyhow::Result;
use config::Config;

pub mod config;
pub mod prompt;

pub fn configure(profile: &String) -> Result<()> {
    let new_config = Config::configure()?;
    new_config.save(profile)?;
    Ok(())
}

pub fn login(profile: &String, role_name: Option<String>) -> Result<()> {
    let config = Config::load(profile)?;
    println!("{:?}", config);
    Ok(())
}
