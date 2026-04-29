use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use crate::auth::jwt::JWT_STORAGE_KEY;

/// HTTP client wrapper that automatically adds Bearer token from SessionStorage
pub struct AuthenticatedClient {
    base_url: String,
}

impl AuthenticatedClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    /// Get the stored JWT token from SessionStorage
    fn get_jwt_token() -> Option<String> {
        web_sys::window()
            .and_then(|w| w.session_storage().ok())
            .flatten()
            .and_then(|storage| storage.get(JWT_STORAGE_KEY).ok())
            .flatten()
    }

    /// Create a GET request with automatic Bearer token injection
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ClientError> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = Request::get(&url);

        if let Some(token) = Self::get_jwt_token() {
            request = request.header("Authorization", &format!("Bearer {}", token));
        }

        let response = request.send().await?;
        let status = response.status();

        if response.ok() {
            let body: T = response.json().await?;
            Ok(body)
        } else {
            Err(ClientError::HttpError {
                status,
                url,
            })
        }
    }

    /// Create a POST request with automatic Bearer token injection
    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ClientError> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = Request::post(&url);

        if let Some(token) = Self::get_jwt_token() {
            request = request.header("Authorization", &format!("Bearer {}", token));
        }

        let response = request.json(body)?.send().await?;
        let status = response.status();

        if response.ok() {
            let body: T = response.json().await?;
            Ok(body)
        } else {
            Err(ClientError::HttpError {
                status,
                url,
            })
        }
    }

    /// Create a PUT request with automatic Bearer token injection
    pub async fn put<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ClientError> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = Request::put(&url);

        if let Some(token) = Self::get_jwt_token() {
            request = request.header("Authorization", &format!("Bearer {}", token));
        }

        let response = request.json(body)?.send().await?;
        let status = response.status();

        if response.ok() {
            let body: T = response.json().await?;
            Ok(body)
        } else {
            Err(ClientError::HttpError {
                status,
                url,
            })
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("HTTP error {status} from {url}")]
    HttpError { status: u16, url: String },

    #[error("Network request failed: {0}")]
    NetworkError(#[from] gloo_net::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(String),
}
