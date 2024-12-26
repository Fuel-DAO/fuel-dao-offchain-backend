use std::{env, sync::Arc};

use candid::Principal;
use ic_agent::{agent::AgentBuilder, Agent, Identity};

const LIVE_AGENT_URL: &str = "https://ic0.app";

const LOCAL_AGENT_URL: &str = "http://localhost:4943";

#[derive(Clone)]
pub struct AgentWrapper(Agent);

impl AgentWrapper {
    pub  fn build(builder_func: impl FnOnce(AgentBuilder) -> AgentBuilder) -> Self {

        // dotenv::dotenv().ok();

        // let live = env::var("BACKEND").unwrap_or("LIVE".to_string()) == "LIVE" ;

        let live = true;

        // let url = "https://ic0.app";
        let url = if live {
            LIVE_AGENT_URL
        } else {
            LOCAL_AGENT_URL
        };

        let mut builder =  Agent::builder().with_url(url);
        builder = builder_func(builder);
        let agent  =builder.build().unwrap();
        
        Self(agent)
    }

    pub fn get_agent(&self) -> &Agent {
        let agent = &self.0;
        agent
    }

    pub fn set_arc_id(&mut self, id: Arc<impl Identity + 'static>) {
        self.0.set_arc_identity(id);
    }

    pub fn principal(&self) -> Result<Principal, String> {
        self.0.get_principal()
    }
}
