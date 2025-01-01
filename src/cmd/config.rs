use anyhow::Result;

use crate::cmd::prompt::{Prompt, Required};

#[derive(Debug)]
pub struct Config {
    pub entra_id_tenant: String,
    pub app_id_uri: String,
    pub session_duration_hours: i32,
    pub chrome_user_data_dir: String,
}

impl Config {
    pub fn configure(profile: &String) -> Result<Config> {
        println!("Configuring profile: {}", profile);

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

    pub fn load(profile: &String) -> Result<Config> {
        Ok(Config {
            app_id_uri: "https://login.microsoftonline.com/".to_string(),
            entra_id_tenant: "common".to_string(),
            session_duration_hours: 1,
            chrome_user_data_dir: "/tmp".to_string(),
        })
    }
}
