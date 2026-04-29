use serde::{Deserialize, Serialize};

/// Represents the state of a KYC flow
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FlowStatus {
    NotStarted,
    InProgress,
    WaitingForReview,
    Approved,
    Rejected,
    Expired,
}

impl std::fmt::Display for FlowStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowStatus::NotStarted => write!(f, "Not Started"),
            FlowStatus::InProgress => write!(f, "In Progress"),
            FlowStatus::WaitingForReview => write!(f, "Waiting for Review"),
            FlowStatus::Approved => write!(f, "Approved"),
            FlowStatus::Rejected => write!(f, "Rejected"),
            FlowStatus::Expired => write!(f, "Expired"),
        }
    }
}

impl FlowStatus {
    pub fn icon(&self) -> &'static str {
        match self {
            FlowStatus::NotStarted => "⏳",
            FlowStatus::InProgress => "🔄",
            FlowStatus::WaitingForReview => "👀",
            FlowStatus::Approved => "✅",
            FlowStatus::Rejected => "❌",
            FlowStatus::Expired => "⏰",
        }
    }

    pub fn badge_classes(&self) -> &'static str {
        match self {
            FlowStatus::NotStarted => "bg-gray-100 text-gray-800",
            FlowStatus::InProgress => "bg-blue-100 text-blue-800",
            FlowStatus::WaitingForReview => "bg-yellow-100 text-yellow-800",
            FlowStatus::Approved => "bg-green-100 text-green-800",
            FlowStatus::Rejected => "bg-red-100 text-red-800",
            FlowStatus::Expired => "bg-gray-100 text-gray-500",
        }
    }
}

/// A single step within a KYC flow
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FlowStep {
    pub id: String,
    pub name: String,
    pub status: StepStatus,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl FlowStep {
    pub fn is_completed(&self) -> bool {
        self.status == StepStatus::Completed
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StepStatus {
    Pending,
    Active,
    Completed,
    Failed,
    Skipped,
}

impl std::fmt::Display for StepStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepStatus::Pending => write!(f, "Pending"),
            StepStatus::Active => write!(f, "Active"),
            StepStatus::Completed => write!(f, "Completed"),
            StepStatus::Failed => write!(f, "Failed"),
            StepStatus::Skipped => write!(f, "Skipped"),
        }
    }
}

impl StepStatus {
    pub fn icon(&self) -> &'static str {
        match self {
            StepStatus::Pending => "1",
            StepStatus::Active => "→",
            StepStatus::Completed => "✓",
            StepStatus::Failed => "✗",
            StepStatus::Skipped => "⊘",
        }
    }

    pub fn ring_color(&self) -> &'static str {
        match self {
            StepStatus::Pending => "ring-gray-300",
            StepStatus::Active => "ring-primary-500 ring-2",
            StepStatus::Completed => "ring-green-500",
            StepStatus::Failed => "ring-red-500",
            StepStatus::Skipped => "ring-gray-300",
        }
    }

    pub fn text_color(&self) -> &'static str {
        match self {
            StepStatus::Pending => "text-gray-500",
            StepStatus::Active => "text-primary-600 font-semibold",
            StepStatus::Completed => "text-green-600",
            StepStatus::Failed => "text-red-600",
            StepStatus::Skipped => "text-gray-400 line-through",
        }
    }
}

/// Complete flow state tracking
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FlowState {
    pub flow_id: String,
    pub flow_type: String,
    pub status: FlowStatus,
    pub current_step: Option<String>,
    pub steps: Vec<FlowStep>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl FlowState {
    pub fn new(flow_id: String, flow_type: String) -> Self {
        Self {
            flow_id,
            flow_type,
            status: FlowStatus::NotStarted,
            current_step: None,
            steps: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn get_active_step(&self) -> Option<&FlowStep> {
        self.steps.iter().find(|s| s.status == StepStatus::Active)
    }

    pub fn get_step_by_id(&self, step_id: &str) -> Option<&FlowStep> {
        self.steps.iter().find(|s| s.id == step_id)
    }

    pub fn update_step_status(&mut self, step_id: &str, status: StepStatus) {
        if let Some(step) = self.steps.iter_mut().find(|s| s.id == step_id) {
            step.status = status;
            self.updated_at = chrono::Utc::now();
        }
    }

    pub fn advance_to_step(&mut self, step_id: String) {
        // Mark current active as completed
        if let Some(current) = self.steps.iter_mut().find(|s| s.status == StepStatus::Active) {
            current.status = StepStatus::Completed;
        }
        
        // Set new step as active
        self.current_step = Some(step_id.clone());
        if let Some(new_step) = self.steps.iter_mut().find(|s| s.id == step_id) {
            new_step.status = StepStatus::Active;
        }
        
        self.status = FlowStatus::InProgress;
        self.updated_at = chrono::Utc::now();
    }
}
