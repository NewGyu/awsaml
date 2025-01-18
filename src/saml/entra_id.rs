use super::{Result, SamlAssertion, SamlAuthRequest, SamlIdProvider};
pub struct EntraIdSamlIdProvider {
    tenant_id: String,
    saml_request: SamlAuthRequest,
}

impl EntraIdSamlIdProvider {
    pub fn new(tenant_id: String, saml_request: SamlAuthRequest) -> Self {
        EntraIdSamlIdProvider {
            tenant_id,
            saml_request,
        }
    }

    fn request_url(&self) -> String {
        format!(
            "https://login.microsoftonline.com/{}/saml2?SAMLRequest={}",
            self.tenant_id,
            self.saml_request.to_string()
        )
    }
}
impl SamlIdProvider for EntraIdSamlIdProvider {
    fn authenticate(&self) -> Result<SamlAssertion> {
        Ok(super::SamlAssertion {})
    }
}
