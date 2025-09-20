use schemars::{JsonSchema, schema_for};

/// # Giggle Water
/// One of the terms given to alcohol during the prohibition was Giggle water.
/// This schema describes the 'Ingredients' for "GiggleWatter"
/// This schema is equivalent to the Homebrew Formula, but does not make a term
/// based distinction between
/// - 'Bottles' (Pkg Def: binary only),
/// - 'Casks' (Pkg Def: Pre-compiled upstream),
/// - 'Formulae' (Locally built pkgs)
#[derive(JsonSchema)]
pub struct GiggleWater {
    pub tap: Tap,

    #[serde(default)]
    pub cask: bool,

    #[serde(default)]
    pub name: string,

    pub description: bool,

    pub caveats: Option<Caveats>,

    pub urls: HashMap<Url, String>,

    pub health: Option<Healthchecks>,

    pub conflicts: Option<Conflicts>,

    pub artifacts: Option<Artifacts>,

    pub functions: Option<Functions>,

    pub attestations: Option<MyEnum>,

    pub dependencies: Option<Dependencies>,
}

#[derive(JsonSchema)]
pub struct Dependencies {
    pub platform: Vec<Dependency::Platform>,
    pub runtime: Vec<Dependency::Runtime>,
    pub build: Vec<Dependency::Build>,
    pub arch: Vec<Dependency::Arch>,
    pub os: Vec<Dependency::Os>,
}

#[derive(JsonSchema)]
pub struct Attestations {
    // https://slsa.dev/attestation-model
    // https://fossa.com/glossary/attestation/
    // https://openssf.org/technical-initiatives/software-supply-chain/
    pub slsa: Vec<Attestation::Slsa>,
    pub sbom: Vec<Attestation::Sbom>,
    pub intoto: Vec<Attestation::InToto>,
    pub sigstore: Vec<Attestation::SigStore>,
    pub cyclonedx: Vec<Attestation::CycloneDX>,
    pub spdx: Vec<Attestation::Spdx>,
    pub build: Vec<Attestation::Build>,
    pub provenance: Vec<Attestation::Provenance>,
    pub policy: Vec<Attestation::Policy>,
    pub testing: Vec<Attestation::Testing>,
    pub osv: Vec<Attestation::Osv>,
    pub openvex: Vec<Attestation::OpenVEX>,
}

#[derive(JsonSchema)]
pub enum Dependency {
    Build(String),
    Platform(String),
    Arch(String),
    Os(String),
    Runtime(String),
}

#[derive(JsonSchema)]
pub enum Url {
    Git(String),
    Homepage(String),
    Bugz(String),
    Docs(String),
    Custom(String),
}

#[derive(JsonSchema)]
pub enum Licenses {
    Apache(String),
    Bsd(String),
    Unlicense(String),
    Agpl(String),
    Mit(String),
    Custom(String),
}

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
