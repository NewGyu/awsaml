use url::Url;

use super::SamlIdProvider;

pub struct EntraIdSamlIdProvider {
    tenant_id: String,
}

impl EntraIdSamlIdProvider {
    pub fn new(tenant_id: String) -> Self {
        EntraIdSamlIdProvider { tenant_id }
    }
}

impl SamlIdProvider for EntraIdSamlIdProvider {
    fn request_base(&self) -> Url {
        Url::parse(&format!(
            "https://login.microsoftonline.com/{}/saml2",
            self.tenant_id
        ))
        .unwrap()
    }
}
