use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait SamlIdProvider {
    fn authenticate(&self) -> Result<SamlResponse>;
}

pub struct SamlResponse {}
pub struct SamlAuthRequest {
    id: Uuid,
    instant: DateTime<Utc>,
    app_id_uri: String,
}
impl SamlAuthRequest {
    pub fn new(app_id_uri: String) -> Self {
        SamlAuthRequest {
            id: Uuid::new_v4(),
            instant: Utc::now(),
            app_id_uri,
        }
    }
}
impl ToString for SamlAuthRequest {
    fn to_string(&self) -> String {
        format!(
            r#"
<samlp:AuthnRequest
  AssertionConsumerServiceURL="{endpoint_url}"
  ID="id_{id}"
  IssueInstant="{timestamp}"
  ProtocolBinding="urn:oasis:names:tc:SAML:2.0:bindings:HTTP-POST"
  Version="2.0"
  xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol">
  <saml:Issuer xmlns:saml="urn:oasis:names:tc:SAML:2.0:assertion">
    {app_uri}
  </saml:Issuer>
  <samlp:NameIDPolicy Format="urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress" />
</samlp:AuthnRequest>"#,
            endpoint_url = "https://example.com/sso",
            id = self.id,
            timestamp = self.instant.to_rfc3339(),
            app_uri = self.app_id_uri
        )
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_saml_auth_request_to_string() {
        let req = SamlAuthRequest::new("https://example.com/app".to_string());
        let s = req.to_string();
        assert_eq!(s, "");
    }
}

pub mod entra_id {
    use super::{Result, SamlIdProvider, SamlResponse};
    pub struct EntraId {}
    impl EntraId {
        pub fn new() -> Self {
            EntraId {}
        }
    }
    impl SamlIdProvider for EntraId {
        fn authenticate(&self) -> Result<SamlResponse> {
            Ok(super::SamlResponse {})
        }
    }
}
