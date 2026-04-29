use crate::api::models::*;
use crate::auth::client::{AuthenticatedClient, ClientError};

const DEFAULT_BFF_BASE_URL: &str = "/bff";

/// BFF API client
pub struct BffClient {
    client: AuthenticatedClient,
}

impl BffClient {
    pub fn new(base_url: Option<String>) -> Self {
        Self {
            client: AuthenticatedClient::new(base_url.unwrap_or_else(|| DEFAULT_BFF_BASE_URL.to_string())),
        }
    }

    /// Start a new KYC flow
    pub async fn start_flow(&self, request: &StartFlowRequest) -> Result<StartFlowResponse, ClientError> {
        self.client.post("/api/kyc/flows", request).await
    }

    /// Get the current state of a KYC flow
    pub async fn get_flow(&self, flow_id: &str) -> Result<GetFlowResponse, ClientError> {
        self.client.get(&format!("/api/kyc/flows/{}", flow_id)).await
    }

    /// Submit data for a specific step
    pub async fn submit_step(
        &self,
        flow_id: &str,
        request: &SubmitStepRequest,
    ) -> Result<SubmitStepResponse, ClientError> {
        self.client
            .post(&format!("/api/kyc/flows/{}/steps/{}", flow_id, request.step_id), request)
            .await
    }

    /// Get user information
    pub async fn get_user_info(&self) -> Result<UserInfoResponse, ClientError> {
        self.client.get("/api/user/info").await
    }

    /// Upload a document for KYC
    pub async fn upload_document(
        &self,
        flow_id: &str,
        step_id: &str,
        file_data: &[u8],
        content_type: &str,
    ) -> Result<SubmitStepResponse, ClientError> {
        // Note: File upload implementation would need multipart support
        // For now, we'll use base64 encoding in JSON
        use base64::{Engine as _, engine::general_purpose};
        
        let encoded = general_purpose::STANDARD.encode(file_data);
        let payload = serde_json::json!({
            "step_id": step_id,
            "data": {
                "document_base64": encoded,
                "content_type": content_type
            }
        });
        
        let request: SubmitStepRequest = serde_json::from_value(payload).map_err(|e| {
            ClientError::JsonError(e.to_string())
        })?;
        
        self.submit_step(flow_id, &request).await
    }
}
