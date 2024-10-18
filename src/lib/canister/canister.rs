use std::sync::Arc;
use candid::Principal;
use ic_agent::{identity::DelegatedIdentity, Identity};
use serde::{Deserialize, Serialize};

use crate::{canister::backend::Backend, identity::{delegated_identity::DelegatedIdentityWire, ic::AgentWrapper}};

use super::BACKEND_ID;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CanistersAuthWire {
    id: DelegatedIdentityWire,
    user_principal: Principal,
    expiry: u64,
    backend_principal: Principal,
    // profile_details: ProfileDetails,
}

impl CanistersAuthWire {
    pub fn canisters(self) -> Result<Canisters<true>, k256::elliptic_curve::Error> {
        let unauth = Canisters::<false>::default();

        let id: DelegatedIdentity = self.id.try_into()?;
        let arc_id = Arc::new(id);

        let mut agent = unauth.agent.clone();
        agent.set_arc_id(arc_id.clone());

        Ok(Canisters {
            agent,
            id: Some(arc_id),
            user_principal: self.user_principal,
            expiry: self.expiry,
            backend_principal: BACKEND_ID,
            // profile_details: Some(self.profile_details),
        })
    }
}

#[derive(Clone)]
pub struct Canisters<const AUTH: bool> {
    agent: AgentWrapper,
    id: Option<Arc<DelegatedIdentity>>,
    user_principal: Principal,
    expiry: u64,
    backend_principal: Principal,
    // profile_details: Option<ProfileDetails>,
}

impl Default for Canisters<false> {
    fn default() -> Self {
        Self {
            agent: AgentWrapper::build(|b| b),
            id: None,
            user_principal: Principal::anonymous(),
            expiry: 0,
            backend_principal: BACKEND_ID,
            // profile_details: None,
        }
    }
}

impl Canisters<true> {
    pub fn authenticated(id: DelegatedIdentity) -> Canisters<true> {
        let expiry = id
            .delegation_chain()
            .iter()
            .fold(u64::MAX, |prev_expiry, del| {
                del.delegation.expiration.min(prev_expiry)
            });
        let id = Arc::new(id);

        Canisters {
            agent: AgentWrapper::build(|b| b.with_arc_identity(id.clone())),
            id: Some(id),
            user_principal: Principal::anonymous(),
            expiry,
            backend_principal: BACKEND_ID,
            // profile_details: None,
        }
    }

    pub fn expiry_ns(&self) -> u64 {
        self.expiry
    }

    pub fn identity(&self) -> &DelegatedIdentity {
        self.id
            .as_ref()
            .expect("Authenticated canisters must have an identity")
    }


    pub fn user_principal(&self) -> Principal {
        self.identity()
            .sender()
            .expect("expect principal to be present")
    }

    pub  fn backend_canister(&self) -> Backend<'_> {
        self.backend()
    }
}

impl<const A: bool> Canisters<A> {
    pub fn backend(&self) -> Backend<'_> {
        let agent = self.agent.get_agent();
        Backend(self.backend_principal, agent)
    }
}

pub async fn do_canister_auth(
    auth: DelegatedIdentityWire,
) -> anyhow::Result<CanistersAuthWire > {
    let id = auth.clone().try_into()?;
    let canisters = Canisters::<true>::authenticated(id);

    // let user = canisters.authenticated_user().await;

    // let profile_details = user.get_profile_details().await?.into();

    let cans_wire = CanistersAuthWire {
        id: auth,
        user_principal: canisters.user_principal,
        expiry: canisters.expiry,
        backend_principal: BACKEND_ID,
    };

    Ok(cans_wire)
}
