use crate::{
    content::attestation::basic_image_attestation::BasicImageAttestationContent,
    statement::attestation::basic_image_attestation::BasicImageAttestationStatement,
    types::{
        defs::{Proof, Statement, Subject},
        error::StatementError,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct BasicImageAttestationProof {
    pub statement: BasicImageAttestationStatement,
    pub signature: String,
}

impl Statement for BasicImageAttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<BasicImageAttestationContent> for BasicImageAttestationProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<BasicImageAttestationContent, crate::types::error::ProofError> {
        Ok(BasicImageAttestationContent {
            id: self.statement.subject.did()?,
            src: self.statement.src.clone(),
            signature: self.signature.clone(),
        })
    }
}