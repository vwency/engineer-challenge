use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FlowResult {
    pub flow: Value,
    pub csrf_token: String,
    pub cookies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PostFlowResult {
    pub data: Value,
    pub cookies: Vec<String>,
}
