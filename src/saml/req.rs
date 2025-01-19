//! Module for handling SAML requests
use chrono::{DateTime, Utc};
use url::Url;
use uuid::Uuid;

use super::EncodedSAML;

pub struct SamlAuthRequest {
    pub id: Uuid,
    pub instant: DateTime<Utc>,
    pub app_id_uri: Url,
    pub callback_to: Url,
}

impl SamlAuthRequest {
    pub fn new(app_id_uri: Url, callback_to: Url) -> Self {
        SamlAuthRequest {
            id: Uuid::new_v4(),
            instant: Utc::now(),
            app_id_uri,
            callback_to,
        }
    }

    fn to_xml(&self) -> String {
        format!(
            r#"
            <samlp:AuthnRequest
                AssertionConsumerServiceURL="{callback_to}"
                ID="id_{id}"
                IssueInstant="{timestamp}"
                ProtocolBinding="urn:oasis:names:tc:SAML:2.0:bindings:HTTP-POST"
                Version="2.0"
                xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol"
            >
                <saml:Issuer xmlns:saml="urn:oasis:names:tc:SAML:2.0:assertion">{req_issuer}</saml:Issuer>
                <samlp:NameIDPolicy Format="urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress" />
            </samlp:AuthnRequest>"#,
            callback_to = self.callback_to,
            id = self.id,
            timestamp = self.instant.to_rfc3339(),
            req_issuer = self.app_id_uri
        )
    }

    pub fn to_encoded_saml(&self) -> EncodedSAML {
        EncodedSAML::from_raw_string(self.to_xml())
    }
}

impl ToString for SamlAuthRequest {
    fn to_string(&self) -> String {
        self.to_xml()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aws::AWS_SAML_CALLBACK;
    use sxd_document::dom::Document;
    use sxd_document::parser as xml_parser;
    use sxd_xpath::{Context, Error, Factory, Value};

    /// test utility function to evaluate an xpath expression with namespaces
    fn evaluate_xpath<'a>(document: &'a Document, xpath: &str) -> Result<Value<'a>, anyhow::Error> {
        let factory = Factory::new();
        let expression = factory.build(xpath)?;
        let expression = expression.ok_or(Error::NoXPath)?;

        let mut context = Context::new();
        context.set_namespace("samlp", "urn:oasis:names:tc:SAML:2.0:protocol");
        context.set_namespace("saml", "urn:oasis:names:tc:SAML:2.0:assertion");

        expression
            .evaluate(&context, document.root())
            .map_err(Into::into)
    }

    #[test]
    fn test_saml_auth_request_to_string() -> anyhow::Result<()> {
        //Arrange
        let saml_req = SamlAuthRequest::new(
            Url::parse("https://example.com")?,
            Url::parse(AWS_SAML_CALLBACK)?,
        );

        //Act
        let xml_string = saml_req.to_string();

        //Assert
        let xml = xml_parser::parse(&xml_string).expect("Could not parse XML");
        let xml = xml.as_document();

        // callback url is set to the AssertionConsumerServiceURL attribute
        let assertion_consumer_service_url_attr =
            evaluate_xpath(&xml, "/samlp:AuthnRequest/@AssertionConsumerServiceURL");
        assert_eq!(
            assertion_consumer_service_url_attr
                .ok()
                .and_then(|v| Some(v.into_string())),
            Some(AWS_SAML_CALLBACK.to_string())
        );

        // app_id_uri is set to the Issuer element
        let issuer_elm = evaluate_xpath(&xml, "/samlp:AuthnRequest/saml:Issuer");
        assert_eq!(
            issuer_elm.ok().and_then(|v| Some(v.into_string())),
            Some("https://example.com/".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_encoded_saml() -> anyhow::Result<()> {
        //Arrange
        let saml_req = SamlAuthRequest::new(
            Url::parse("https://example.com")?,
            Url::parse(AWS_SAML_CALLBACK)?,
        );

        //Act
        let encoded_saml = saml_req.to_encoded_saml();
        //        println!("{:?}", result);
        //Assert
        assert_eq!(encoded_saml.to_raw_string()?, saml_req.to_string());
        Ok(())
    }
}
