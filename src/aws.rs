mod assume_role;
mod config;
pub mod credentials;

pub use assume_role::assume_role_with_saml;
pub use config::Config;

pub const AWS_SAML_CALLBACK: &str = "https://signin.aws.amazon.com/saml";
