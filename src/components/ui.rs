use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub full_width: bool,
    pub children: Children,
}

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Ghost,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let base_classes = "inline-flex items-center justify-center px-4 py-2 text-sm font-medium rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 transition-all duration-200 transform hover:-translate-y-0.5";
    
    let variant_classes = match props.variant {
        ButtonVariant::Primary => "bg-primary-600 hover:bg-primary-700 text-white focus:ring-primary-500 border border-transparent hover:shadow-lg btn-primary",
        ButtonVariant::Secondary => "bg-white hover:bg-gray-50 text-gray-700 focus:ring-primary-500 border border-gray-300 shadow-sm hover:shadow-md",
        ButtonVariant::Success => "bg-green-600 hover:bg-green-700 text-white focus:ring-green-500 border border-transparent hover:shadow-lg",
        ButtonVariant::Danger => "bg-red-600 hover:bg-red-700 text-white focus:ring-red-500 border border-transparent hover:shadow-lg",
        ButtonVariant::Ghost => "bg-transparent hover:bg-gray-100 text-gray-700 focus:ring-gray-500 border border-transparent shadow-none",
    };

    let width_class = if props.full_width { "w-full transform-none" } else { "" };

    html! {
        <button
            class={classes!(
                base_classes,
                variant_classes,
                width_class,
                props.disabled.then_some("opacity-50 cursor-not-allowed transform-none hover:translate-y-0")
            )}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
        >
            { for props.children.iter() }
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub label: String,
    pub value: String,
    pub oninput: Callback<String>,
    #[prop_or(String::from("text"))]
    pub input_type: String,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or_default]
    pub icon: Option<Html>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let oninput = props.oninput.clone();
    let input_oninput = Callback::from(move |e: InputEvent| {
        let target: web_sys::HtmlInputElement = e.target_unchecked_into();
        oninput.emit(target.value());
    });

    let has_error = props.error.is_some();
    let border_classes = if has_error {
        "border-red-300 focus:ring-red-500 focus:border-red-500"
    } else {
        "border-gray-300 focus:ring-primary-500 focus:border-primary-500 hover:border-primary-400"
    };

    html! {
        <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-1">
                { &props.label }
                if props.required {
                    <span class="text-red-500 ml-1">{"*"}</span>
                }
            </label>
            <div class="relative rounded-md shadow-sm">
                if let Some(icon) = &props.icon {
                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        { icon.clone() }
                    </div>
                }
                <input
                    type={props.input_type.clone()}
                    class={classes!(
                        "appearance-none", "block", "w-full", "rounded-lg", "border", "py-2.5", "placeholder-gray-400", "text-sm", "transition-all", "duration-200",
                        border_classes,
                        props.icon.is_some().then_some("pl-10 pr-3").unwrap_or("px-3 pr-10")
                    )}
                    value={props.value.clone()}
                    placeholder={props.placeholder.clone().unwrap_or_default()}
                    required={props.required}
                    oninput={input_oninput}
                    aria-invalid={has_error.to_string()}
                />
                if has_error {
                    <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                        <svg class="h-5 w-5 text-red-500" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
                        </svg>
                    </div>
                }
            </div>
            if let Some(error) = &props.error {
                <p class="mt-1 text-sm text-red-600 flex items-center animate-fade-in">
                    <svg class="w-4 h-4 mr-1 flex-shrink-0" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
                    </svg>
                    { error }
                </p>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    pub message: String,
    #[prop_or(AlertVariant::Info)]
    pub variant: AlertVariant,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
}

#[derive(Clone, PartialEq)]
pub enum AlertVariant {
    Info,
    Success,
    Warning,
    Error,
}

impl AlertVariant {
    pub fn icon(&self) -> Html {
        match self {
            AlertVariant::Info => html! {
                <svg class="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                </svg>
            },
            AlertVariant::Success => html! {
                <svg class="h-5 w-5 text-green-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                </svg>
            },
            AlertVariant::Warning => html! {
                <svg class="h-5 w-5 text-yellow-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
                </svg>
            },
            AlertVariant::Error => html! {
                <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
                </svg>
            },
        }
    }

    pub fn container_classes(&self) -> &'static str {
        match self {
            AlertVariant::Info => "bg-blue-50 border-blue-200",
            AlertVariant::Success => "bg-green-50 border-green-200",
            AlertVariant::Warning => "bg-yellow-50 border-yellow-200",
            AlertVariant::Error => "bg-red-50 border-red-200",
        }
    }

    pub fn text_classes(&self) -> &'static str {
        match self {
            AlertVariant::Info => "text-blue-800",
            AlertVariant::Success => "text-green-800",
            AlertVariant::Warning => "text-yellow-800",
            AlertVariant::Error => "text-red-800",
        }
    }
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    html! {
        <div class={classes!("rounded-lg", "border", "p-4", "animate-scale-in", props.variant.container_classes())}>
            <div class="flex">
                <div class="flex-shrink-0">
                    { props.variant.icon() }
                </div>
                <div class="ml-3 flex-1">
                    if let Some(title) = &props.title {
                        <h3 class={classes!("text-sm", "font-medium", props.variant.text_classes())}>{ title }</h3>
                    }
                    <p class={classes!("text-sm", props.variant.text_classes())}>{ &props.message }</p>
                </div>
                if let Some(onclose) = &props.onclose {
                    <div class="ml-auto pl-3">
                        <div class="-mx-1.5 -my-1.5">
                            <button
                                class={classes!("inline-flex", "rounded-lg", "p-1.5", "hover:bg-white/50", "transition-all", "duration-200", props.variant.text_classes())}
                                onclick={onclose.reform(|_| ())}
                            >
                                <span class="sr-only">{"Dismiss"}</span>
                                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                                </svg>
                            </button>
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}

/// A card component for grouping content
#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub children: Children,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub footer: Option<Html>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden card-hover-lift">
            if let Some(title) = &props.title {
                <div class="bg-gradient-to-r from-primary-600 to-primary-700 px-6 py-4">
                    <h3 class="text-lg font-semibold text-white">{ title }</h3>
                </div>
            }
            <div class="p-6">
                { for props.children.iter() }
            </div>
            if let Some(footer) = &props.footer {
                <div class="bg-gray-50 px-6 py-4 flex justify-end space-x-3 border-t border-gray-200">
                    { footer.clone() }
                </div>
            }
        </div>
    }
}
