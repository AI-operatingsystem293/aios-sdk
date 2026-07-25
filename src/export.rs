use crate::agent::Agent;

/// Every AIOS plugin exports exactly one function named
/// `create_agent` that returns the plugin implementation.
pub trait AgentFactory {
    fn create() -> Box<dyn Agent>;
}
