use crate::agent::Agent;
use crate::manifest::AgentManifest;

pub trait AgentPlugin {
    fn manifest(&self) -> AgentManifest;

    fn create(&self) -> Box<dyn Agent>;
}
