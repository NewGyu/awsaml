use std::path::Path;

use anyhow::Result;
use ini::Ini;

use crate::{
    aws,
    cmd::prompt::{Prompt, Required},
};

#[derive(Debug)]
pub struct Config {
    pub entra_id_tenant: String,
    pub app_id_uri: String,
    pub session_duration_hours: i32,
    pub chrome_user_data_dir: String,
}

impl Config {
    /// create a new Config<br>
    /// ask some questions to the user
    pub fn configure() -> Result<Config> {
        Ok(Config {
            entra_id_tenant: Prompt {
                msg: "Enter the entra ID tenant".to_string(),
                required: Required::Yes(None),
            }
            .get_value()?,
            app_id_uri: Prompt {
                msg: "Enter the app ID URI".to_string(),
                required: Required::Yes(None),
            }
            .get_value()?,
            session_duration_hours: Prompt {
                msg: "Enter the session duration in hours".to_string(),
                required: Required::Yes(Some("6".to_string())),
            }
            .get_value()?,
            chrome_user_data_dir: Prompt {
                msg: "Enter the Chrome user data directory".to_string(),
                required: Required::Yes(Some("/tmp".to_string())),
            }
            .get_value()?,
        })
    }

    /// save the configuration to a ini file
    pub fn save(&self, profile: &String) -> Result<()> {
        let config_file_path = aws::config::file_path();
        let mut config_ini = Config::load_config_file(&config_file_path)?.unwrap_or(Ini::new());

        config_ini
            .with_section(Some(profile))
            .set(config_keys::ENTRA_ID_TENANT, &self.entra_id_tenant)
            .set(config_keys::APP_ID_URI, &self.app_id_uri)
            .set(
                config_keys::SESSION_DURATION_HOURS,
                &self.session_duration_hours.to_string(),
            )
            .set(
                config_keys::CHROME_USER_DATA_DIR,
                &self.chrome_user_data_dir,
            );

        config_ini.write_to_file(&config_file_path)?;
        println!("Configuration saved to {}", &config_file_path);
        Ok(())
    }

    fn load_config_file(filepath: &String) -> Result<Option<Ini>> {
        // does the file exist?
        if !Path::new(filepath).exists() {
            return Ok(None);
        }

        let config_file = Ini::load_from_file(filepath)?;
        Ok(Some(config_file))
    }

    pub fn load(profile: &String) -> Result<Config> {
        let config_file_path = aws::config::file_path();
        if let Some(config_ini) = Config::load_config_file(&config_file_path)? {
            if let Some(section) = config_ini.section(Some(profile)) {
                Ok(Config {
                    entra_id_tenant: section
                        .get(config_keys::ENTRA_ID_TENANT)
                        .unwrap_or("")
                        .to_string(),
                    app_id_uri: section
                        .get(config_keys::APP_ID_URI)
                        .unwrap_or("")
                        .to_string(),
                    session_duration_hours: section
                        .get(config_keys::SESSION_DURATION_HOURS)
                        .unwrap_or("0")
                        .parse::<i32>()?,
                    chrome_user_data_dir: section
                        .get(config_keys::CHROME_USER_DATA_DIR)
                        .unwrap_or("")
                        .to_string(),
                })
            } else {
                Err(anyhow::anyhow!("Profile not found"))
            }
        } else {
            Err(anyhow::anyhow!("Config file not found"))
        }
    }
}

mod config_keys {
    pub const ENTRA_ID_TENANT: &str = "entra_id_tenant";
    pub const APP_ID_URI: &str = "app_id_uri";
    pub const SESSION_DURATION_HOURS: &str = "session_duration_hours";
    pub const CHROME_USER_DATA_DIR: &str = "chrome_user_data_dir";
}
