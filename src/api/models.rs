use serde::{Deserialize, Serialize};
use crate::state::flow::{FlowState, FlowStatus};

/// API request to start a KYC flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartFlowRequest {
    pub flow_type: String,
    pub user_id: Option<String>,
}

/// API response when starting a KYC flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartFlowResponse {
    pub flow_id: String,
    pub session_id: String,
    pub initial_step: Option<String>,
}

/// API request to submit step data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitStepRequest {
    pub step_id: String,
    pub data: serde_json::Value,
}

/// API response for step submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitStepResponse {
    pub success: bool,
    pub next_step: Option<String>,
    pub flow_status: FlowStatus,
}

/// API response for getting flow state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFlowResponse {
    pub flow: FlowState,
}

/// API response for user info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub user_id: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub kyc_status: Option<FlowStatus>,
}

/// Generic API error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}
