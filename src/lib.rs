/// <https://w3id.org/did-resolution/v1>
#[cfg(feature = "did-resolution")]
pub const DID_RESOLUTION_V1: &str = include_str!("../w3c-did-resolution-v1.jsonld");

/// <https://www.w3.org/TR/did-core/>
#[cfg(feature = "did-core")]
pub const DID_V1: &str = include_str!("../w3c-did-v1.jsonld");

/// <https://www.w3.org/TR/vc-data-model/>
#[cfg(feature = "vc-data-model")]
pub const CREDENTIALS_V1: &str = include_str!("../w3c-verifiable-credentials-v1.jsonld");

/// <https://www.w3.org/2018/credentials/examples/v1>
#[cfg(feature = "vc-data-model")]
pub const CREDENTIALS_EXAMPLES_V1: &str =
    include_str!("../w3c-verifiable-credentials-examples-v1.jsonld");

/// <https://schema.org/>
#[cfg(feature = "schema")]
pub const SCHEMA_ORG: &str = include_str!("../schema.org.jsonld");