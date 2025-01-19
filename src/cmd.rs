pub mod config;
pub mod prompt;

use anyhow::Result;
use awsaml::aws::{assume_role_with_saml, AWS_SAML_CALLBACK};
use awsaml::saml::entra_id::EntraIdSamlIdProvider;
use awsaml::saml::{ChromeSamlAgent, SamlAuthRequest};
use config::Config;
use url::Url;

/// Configure the AWSaml CLI.
/// This will prompt the user for the necessary configuration values
/// and save them to '~/.aws/config' file.
pub fn configure(profile: &String) -> Result<()> {
    let new_config = Config::configure()?;
    new_config.save(profile)?;
    Ok(())
}

/// Login to Entrata ID and retrieve the SAML assertion
/// to authenticate with AWS.
///
/// This will open a browser tab to the Entrata ID login page
/// and call `assumeRoleWithSAML` to acquire AWS credentials.
///
/// Aquired credentials will be saved to `~/.aws/credentials` file.
pub fn login(profile: &String, _role_name: Option<String>) -> Result<()> {
    let config = Config::load(profile)?;
    let saml_req = SamlAuthRequest::new(
        Url::parse(&config.app_id_uri)?,
        Url::parse(AWS_SAML_CALLBACK)?,
    );
    let entra_id = EntraIdSamlIdProvider::new(config.entra_id_tenant.to_string());
    let mut agent = ChromeSamlAgent::new(Box::new(entra_id), Url::parse(AWS_SAML_CALLBACK)?);
    agent.launch_browser_tab()?;
    let saml_res = agent.process_saml_request(saml_req)?;
    println!("{:?}", &saml_res);
    assume_role_with_saml(saml_res);
    Ok(())
}
