use crate::types::{
    defs::{Content, Subject},
    enums::subject::Subjects,
    error::ContentError,
};
use chrono::{SecondsFormat, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct RedditVerificationContent {
    pub handle: String,
    pub subject: Subjects,
    pub statement: String,
    pub signature: String,
}

impl Content for RedditVerificationContent {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            "https://spec.rebase.xyz/contexts/v1"
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "handle".to_string(),
            serde_json::Value::String(self.handle.clone()),
        );

        evidence_map.insert(
            "timestamp".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["RedditVerificationMessage".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.subject.did()?,
            "sameAs": format!("https://reddit.com/user/{}/", self.handle)
        }))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_owned(),
            "RedditVerification".to_owned(),
        ])
    }
}