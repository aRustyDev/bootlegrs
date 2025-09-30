use crate::{attestations, recipe, rotgut, speakeasy, still};
use schemars::JsonSchema;

#[derive(JsonSchema)]
pub struct Attestations {
    // https://slsa.dev/attestation-model
    // https://fossa.com/glossary/attestation/
    // https://openssf.org/technical-initiatives/software-supply-chain/
    pub osv: Vec<Attestation>,
    pub slsa: Vec<Attestation>,
    pub sbom: Vec<Attestation>,
    pub spdx: Vec<Attestation>,
    pub build: Vec<Attestation>,
    pub policy: Vec<Attestation>,
    pub intoto: Vec<Attestation>,
    pub testing: Vec<Attestation>,
    pub openvex: Vec<Attestation>,
    pub sigstore: Vec<Attestation>,
    pub cyclonedx: Vec<Attestation>,
    pub provenance: Vec<Attestation>,
}

#[derive(JsonSchema)]
pub struct SlsaAttestation {}

#[derive(JsonSchema)]
pub struct SbomAttestation {}

#[derive(JsonSchema)]
pub struct InTotoAttestation {}

#[derive(JsonSchema)]
pub struct SigStoreAttestation {}

#[derive(JsonSchema)]
pub struct CycloneDXAttestation {}

#[derive(JsonSchema)]
pub struct SpdxAttestation {}

#[derive(JsonSchema)]
pub struct BuildAttestation {}

#[derive(JsonSchema)]
pub struct ProvenanceAttestation {}

#[derive(JsonSchema)]
pub struct PolicyAttestation {}

#[derive(JsonSchema)]
pub struct TestingAttestation {}

#[derive(JsonSchema)]
pub struct OsvAttestation {}

#[derive(JsonSchema)]
pub struct OpenVEXAttestation {}

#[derive(JsonSchema)]
pub enum Attestation {
    Slsa(String),
    Sbom(String),
    InToto(String),
    SigStore(String),
    CycloneDX(String),
    Spdx(String),
    Build(String),
    Provenance(String),
    Policy(String),
    Testing(String),
    Osv(String),
    OpenVEX(String),
}
