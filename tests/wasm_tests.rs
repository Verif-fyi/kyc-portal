use wasm_bindgen_test::*;
use veriffyi::state::flow::{FlowState, FlowStatus, StepStatus};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_flow_state_creation() {
    let flow = FlowState::new("test-flow-id".to_string(), "phone_otp".to_string());
    assert_eq!(flow.flow_id, "test-flow-id");
    assert_eq!(flow.flow_type, "phone_otp");
    assert_eq!(flow.status, FlowStatus::NotStarted);
    assert!(flow.current_step.is_none());
    assert!(flow.steps.is_empty());
}

#[wasm_bindgen_test]
fn test_flow_state_step_advancement() {
    let mut flow = FlowState::new("test-flow-id".to_string(), "phone_otp".to_string());
    flow.advance_to_step("step-1".to_string());
    assert_eq!(flow.status, FlowStatus::InProgress);
    assert_eq!(flow.current_step, Some("step-1".to_string()));
}

#[wasm_bindgen_test]
fn test_flow_state_update_step_status() {
    let mut flow = FlowState::new("test-flow-id".to_string(), "phone_otp".to_string());
    flow.advance_to_step("step-1".to_string());
    flow.update_step_status("step-1", StepStatus::Completed);
    
    let step = flow.get_step_by_id("step-1");
    assert!(step.is_some());
    assert_eq!(step.unwrap().status, StepStatus::Completed);
}

#[wasm_bindgen_test]
fn test_flow_status_display() {
    assert_eq!(FlowStatus::NotStarted.to_string(), "Not Started");
    assert_eq!(FlowStatus::InProgress.to_string(), "In Progress");
    assert_eq!(FlowStatus::Approved.to_string(), "Approved");
    assert_eq!(FlowStatus::Rejected.to_string(), "Rejected");
}
