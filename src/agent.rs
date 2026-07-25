use crate::{
    context::Context,
    response::Response,
};

pub trait Agent: Send + Sync {
    fn name(&self) -> &'static str;

    fn version(&self) -> &'static str;

    fn author(&self) -> &'static str;

    fn description(&self) -> &'static str;

    fn capabilities(&self) -> Vec<&'static str>;

    fn execute(
        &self,
        capability: &str,
        input: &str,
        context: &Context,
    ) -> Response;
}
