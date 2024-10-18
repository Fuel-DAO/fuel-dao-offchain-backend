use ic_agent::identity::Secp256k1Identity;

use super::delegated_identity::DelegatedIdentityWire;

pub fn extract_identity(secret: Option<k256::SecretKey>) -> Option<DelegatedIdentityWire> {
    let base_identity = if let Some(identity) = secret {
        Secp256k1Identity::from_private_key(identity)
    } else {
        return None;
    };
    Some(DelegatedIdentityWire::delegate(&base_identity))
}