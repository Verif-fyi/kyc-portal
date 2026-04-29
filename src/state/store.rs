use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use yew::prelude::*;
use super::flow::FlowState;

/// Application state shared across components via Yew Context
#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub current_flow: Option<FlowState>,
    pub flows: HashMap<String, FlowState>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_flow: None,
            flows: HashMap::new(),
            user_id: None,
            session_id: None,
        }
    }

    pub fn set_flow(&mut self, flow_id: String, flow: FlowState) {
        self.flows.insert(flow_id.clone(), flow);
        self.current_flow = self.flows.get(&flow_id).cloned();
    }

    pub fn get_flow(&self, flow_id: &str) -> Option<&FlowState> {
        self.flows.get(flow_id)
    }

    pub fn clear_flow(&mut self) {
        self.current_flow = None;
    }
}

/// Yew Context wrapper for AppState
#[derive(Clone)]
pub struct StateContext(pub Rc<RefCell<AppState>>);

impl PartialEq for StateContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[hook]
pub fn use_state_context() -> Rc<RefCell<AppState>> {
    let ctx = use_context::<StateContext>();
    match ctx {
        Some(ctx) => ctx.0.clone(),
        None => {
            let state = Rc::new(RefCell::new(AppState::new()));
            state.clone()
        }
    }
}
