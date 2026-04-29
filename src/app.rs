use yew::prelude::*;
use yew_router::prelude::*;

use crate::auth::jwt::extract_jwt_from_url;
use crate::components::layout::Layout;
use crate::components::flow::FlowPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/kyc/{flow_id}")]
    KycFlow { flow_id: String },
    #[at("/kyc/{flow_id}/step/{step_id}")]
    KycStep { flow_id: String, step_id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <LandingPage /> },
        Route::KycFlow { flow_id } => html! { <FlowPage flow_id={flow_id} /> },
        Route::KycStep { flow_id, step_id } => html! { <FlowPage flow_id={flow_id} step_id={step_id} /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

#[function_component(LandingPage)]
fn landing_page() -> Html {
    html! {
        <div class="max-w-4xl mx-auto">
            // Hero Section with animation
            <div class="text-center mb-12 animate-fade-in-down">
                <div class="flex justify-center mb-6">
                    <div class="bg-gradient-to-br from-primary-600 to-primary-700 rounded-2xl p-6 shadow-xl hover:shadow-2xl transition-shadow duration-300">
                        <svg class="w-16 h-16 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                        </svg>
                    </div>
                </div>
                <h1 class="text-4xl font-bold text-gray-900 mb-3">{"Welcome to KYC Portal"}</h1>
                <p class="text-lg text-gray-600 mb-2">{"Secure identity verification made simple"}</p>
                <p class="text-sm text-gray-500">{"Complete your identity verification in just a few steps"}</p>
            </div>

            // Feature Cards with staggered animations
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
                <div class="feature-card-1 animate-fade-in-up">
                    <FeatureCard
                        icon={html! {
                            <svg class="w-8 h-8 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                            </svg>
                        }}
                        title={"Secure & Private"}
                        description={"Your data is encrypted and stored securely"}
                    />
                </div>
                <div class="feature-card-2 animate-fade-in-up">
                    <FeatureCard
                        icon={html! {
                            <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                            </svg>
                        }}
                        title={"Fast Verification"}
                        description={"Complete verification in under 5 minutes"}
                    />
                </div>
                <div class="feature-card-3 animate-fade-in-up">
                    <FeatureCard
                        icon={html! {
                            <svg class="w-8 h-8 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"/>
                            </svg>
                        }}
                        title={"Compliant"}
                        description={"GDPR and regulatory compliant processes"}
                    />
                </div>
            </div>

            // CTA Section with improved hover effects
            <div class="bg-white rounded-xl shadow-lg border border-gray-200 p-8 text-center hover:shadow-xl transition-shadow duration-300 animate-scale-in">
                <h2 class="text-2xl font-semibold text-gray-900 mb-3">{"Ready to Get Started?"}</h2>
                <p class="text-gray-600 mb-6">{"Enter your verification link or contact your administrator to begin the KYC process."}</p>
                <div class="flex flex-col sm:flex-row justify-center gap-4">
                    <a href="/kyc/new" class="btn-primary inline-flex items-center justify-center px-6 py-3 border border-transparent text-base font-medium rounded-lg text-white bg-primary-600 hover:bg-primary-700 shadow-md transition-all duration-200 hover:shadow-lg transform hover:-translate-y-0.5">
                        {"Start New Verification"}
                    </a>
                    <a href="/help" class="inline-flex items-center justify-center px-6 py-3 border border-gray-300 text-base font-medium rounded-lg text-gray-700 bg-white hover:bg-gray-50 shadow-sm transition-all duration-200 hover:shadow-md transform hover:-translate-y-0.5">
                        {"Learn More"}
                    </a>
                </div>
            </div>

            // Trust Badges
            <div class="mt-8 flex flex-wrap justify-center items-center gap-6 text-sm text-gray-500 animate-fade-in">
                <div class="flex items-center space-x-2 hover:text-green-600 transition-colors duration-200 cursor-help">
                    <svg class="w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                    </svg>
                    <span>{"256-bit SSL Encryption"}</span>
                </div>
                <div class="flex items-center space-x-2 hover:text-green-600 transition-colors duration-200 cursor-help">
                    <svg class="w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                    </svg>
                    <span>{"SOC 2 Certified"}</span>
                </div>
                <div class="flex items-center space-x-2 hover:text-green-600 transition-colors duration-200 cursor-help">
                    <svg class="w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                    </svg>
                    <span>{"GDPR Compliant"}</span>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct FeatureCardProps {
    icon: Html,
    title: String,
    description: String,
}

#[function_component(FeatureCard)]
fn feature_card(props: &FeatureCardProps) -> Html {
    html! {
        <div class="bg-white rounded-xl shadow-md border border-gray-200 p-6 text-center card-hover-lift hover:shadow-lg transition-all duration-200">
            <div class="flex justify-center mb-4">
                { props.icon.clone() }
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">{ &props.title }</h3>
            <p class="text-sm text-gray-600">{ &props.description }</p>
        </div>
    }
}

#[function_component(NotFoundPage)]
fn not_found_page() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center py-16 animate-scale-in">
            <div class="bg-white rounded-xl shadow-lg border border-gray-200 p-8 text-center max-w-md card-hover-lift">
                <div class="flex justify-center mb-4">
                    <div class="bg-red-100 rounded-full p-3 animate-pulse">
                        <svg class="w-12 h-12 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                    </div>
                </div>
                <h1 class="text-3xl font-bold text-gray-900 mb-2">{"404"}</h1>
                <p class="text-gray-600 mb-6">{"The page you are looking for does not exist."}</p>
                <Link<Route> to={Route::Home} classes="btn-primary inline-flex items-center justify-center px-6 py-3 border border-transparent text-base font-medium rounded-lg text-white bg-primary-600 hover:bg-primary-700 shadow-md transition-all duration-200 hover:shadow-lg transform hover:-translate-y-0.5">
                    {"Go Home"}
                </Link<Route>>
            </div>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    extract_jwt_from_url();

    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}

pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
