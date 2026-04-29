use std::rc::Rc;
use yew::prelude::*;
use crate::state::flow::FlowState;
use crate::components::layout::LoadingSpinner;

/// Props for the FlowPage component
#[derive(Properties, PartialEq)]
pub struct FlowPageProps {
    pub flow_id: String,
    #[prop_or_default]
    pub step_id: Option<String>,
}

/// Main flow page component that renders the current step
#[function_component(FlowPage)]
pub fn flow_page(props: &FlowPageProps) -> Html {
    let flow_state = use_state(|| Option::<Rc<FlowState>>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    {
        let flow_id = props.flow_id.clone();
        let flow_state = flow_state.clone();
        let loading = loading.clone();

        use_effect_with(flow_id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // TODO: Replace with actual API call
                // For now, create a mock flow state
                let mock_flow = FlowState::new(flow_id.clone(), "phone_otp".to_string());
                flow_state.set(Some(Rc::new(mock_flow)));
                loading.set(false);
            });
        });
    }

    if *loading {
        return html! { <LoadingSpinner /> };
    }

    if let Some(err) = error.as_ref() {
        return html! {
            <div class="p-4 bg-red-50 border-l-4 border-red-500 rounded-md">
                <div class="flex">
                    <div class="flex-shrink-0">
                        <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
                        </svg>
                    </div>
                    <div class="ml-3">
                        <p class="text-sm text-red-700">{ err }</p>
                    </div>
                </div>
            </div>
        };
    }

    let flow = match flow_state.as_ref() {
        Some(f) => f.clone(),
        None => return html! { <p>{"Flow not found"}</p> },
    };

    html! {
        <div class="space-y-8">
            <FlowHeader flow={flow.clone()} />
            <FlowStepper flow={flow.clone()} />
            if let Some(step_id) = &props.step_id {
                <StepCard step_id={step_id.clone()} />
            } else if let Some(current_step) = &flow.current_step {
                <StepCard step_id={current_step.clone()} />
            } else {
                <NoActiveStep />
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FlowHeaderProps {
    pub flow: Rc<FlowState>,
}

#[function_component(FlowHeader)]
fn flow_header(props: &FlowHeaderProps) -> Html {
    let status_badge_classes = props.flow.status.badge_classes();

    html! {
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 animate-fade-in-down card-hover-lift">
            <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                <div class="flex items-center space-x-4">
                    <div class="bg-primary-100 rounded-lg p-3">
                        <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="text-lg font-semibold text-gray-900">{ &props.flow.flow_type }</h2>
                        <p class="text-sm text-gray-500">{ format!("ID: {}", &props.flow.flow_id) }</p>
                    </div>
                </div>
                <span class={classes!("inline-flex", "items-center", "px-3", "py-1", "text-sm", "font-medium", "rounded-full", status_badge_classes, "badge-pulse")}>
                    { props.flow.status.icon() }
                    <span class="ml-2">{ &props.flow.status.to_string() }</span>
                </span>
            </div>
        </div>
    }
}

/// Horizontal step indicator for the KYC flow
#[derive(Properties, PartialEq)]
pub struct FlowStepperProps {
    pub flow: Rc<FlowState>,
}

#[function_component(FlowStepper)]
fn flow_stepper(props: &FlowStepperProps) -> Html {
    let total = props.flow.steps.len() as f32;
    let completed = props.flow.steps.iter().filter(|s| s.is_completed()).count() as f32;
    let progress_pct = if total > 0.0 { (completed / total) * 100.0 } else { 0.0 };
    let progress_style = format!("width: {}%;", progress_pct);

    html! {
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 animate-fade-in">
            <h3 class="text-sm font-medium text-gray-700 mb-6">{"Verification Steps"}</h3>
            <div class="relative">
                // Progress bar background
                <div class="absolute top-4 left-0 right-0 h-0.5 bg-gray-200 rounded-full"></div>
                // Progress bar fill with glow effect
                <div
                    class="absolute top-4 left-0 h-0.5 bg-primary-600 transition-all duration-500 rounded-full progress-bar-glow"
                    style={progress_style}
                ></div>
                // Step indicators
                <div class="relative flex justify-between">
                    { for props.flow.steps.iter().enumerate().map(|(_i, step)| {
                        let step_classes = step.status.text_color();
                        let ring_classes = step.status.ring_color();

                        html! {
                            <div key={step.id.clone()} class="flex flex-col items-center flex-1">
                                <div class={classes!("w-8", "h-8", "rounded-full", "flex", "items-center", "justify-center", "text-xs", "font-medium", "bg-white", "border-2", ring_classes, step_classes, "animate-scale-in")}>
                                    { step.status.icon() }
                                </div>
                                <span class={classes!("mt-2", "text-xs", "text-center", step_classes)}>
                                    { &step.name }
                                </span>
                            </div>
                        }
                    }) }
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct StepCardProps {
    pub step_id: String,
}

#[function_component(StepCard)]
pub fn step_card(props: &StepCardProps) -> Html {
    html! {
        <div class="bg-white rounded-xl shadow-lg border border-gray-200 overflow-hidden card-hover-lift animate-scale-in">
            // Card Header
            <div class="bg-gradient-to-r from-primary-600 to-primary-700 px-6 py-4">
                <div class="flex items-center space-x-3">
                    <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                    </svg>
                    <h3 class="text-lg font-semibold text-white">{ format!("Step: {}", props.step_id) }</h3>
                </div>
            </div>

            // Card Body
            <div class="p-6">
                <p class="text-gray-600 mb-6">{"Please complete this verification step."}</p>
                
                // Placeholder for actual step content - will be replaced based on step type
                <div class="bg-gray-50 rounded-lg border-2 border-dashed border-gray-300 p-8 text-center skeleton">
                    <svg class="w-12 h-12 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                    </svg>
                    <p class="text-gray-500 font-medium">{"Step content will be rendered here"}</p>
                    <p class="text-sm text-gray-400 mt-2">{"Upload documents, take photos, or enter information as required"}</p>
                </div>
            </div>

            // Card Footer with action buttons
            <div class="bg-gray-50 px-6 py-4 flex justify-end space-x-3 border-t border-gray-200">
                <button type="button" class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-all duration-200 transform hover:-translate-y-0.5 hover:shadow-sm">
                    {"Save Draft"}
                </button>
                <button type="button" class="btn-primary px-4 py-2 text-sm font-medium text-white bg-primary-600 border border-transparent rounded-lg hover:bg-primary-700 transition-all duration-200 transform hover:-translate-y-0.5 hover:shadow-lg">
                    {"Continue"}
                </button>
            </div>
        </div>
    }
}

#[function_component(NoActiveStep)]
fn no_active_step() -> Html {
    html! {
        <div class="bg-white rounded-xl shadow-lg border border-gray-200 p-8 text-center animate-scale-in card-hover-lift">
            <div class="flex justify-center mb-4">
                <div class="bg-gray-100 rounded-full p-4">
                    <svg class="w-12 h-12 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"/>
                    </svg>
                </div>
            </div>
            <h3 class="text-xl font-semibold text-gray-900 mb-2">{"No Active Step"}</h3>
            <p class="text-gray-600 max-w-md mx-auto">{"This flow has no active step. Please start the flow to begin verification."}</p>
        </div>
    }
}
