use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

/// Main layout component with header and content area
#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="min-h-screen bg-gradient-to-br from-gray-50 via-white to-primary-50 flex flex-col">
            <Header />
            <main class="flex-1 max-w-7xl mx-auto w-full py-8 sm:px-6 lg:px-8 animate-fade-in">
                { for props.children.iter() }
            </main>
            <Footer />
        </div>
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header class="bg-gradient-to-r from-primary-700 to-primary-600 shadow-lg animate-fade-in-down">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    <div class="flex items-center space-x-3">
                        // Logo placeholder - shield icon
                        <div class="bg-white/20 backdrop-blur-sm rounded-lg p-2 hover:bg-white/30 transition-colors duration-200">
                            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                            </svg>
                        </div>
                        <div>
                            <h1 class="text-xl font-bold text-white tracking-tight">{"KYC Portal"}</h1>
                            <p class="text-xs text-primary-100">{"Identity Verification Platform"}</p>
                        </div>
                    </div>
                    <div class="flex items-center space-x-4">
                        <div class="hidden sm:flex items-center space-x-2 bg-white/10 backdrop-blur-sm rounded-full px-4 py-2">
                            <svg class="w-4 h-4 text-green-300" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span class="text-sm text-white font-medium">{"Secure & Encrypted"}</span>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class="bg-gray-800 border-t border-gray-700 mt-auto">
            <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                <div class="flex flex-col sm:flex-row justify-between items-center space-y-2 sm:space-y-0">
                    <p class="text-sm text-gray-400">
                        {"© 2026 Veriffyi - Secure KYC Verification Platform"}
                    </p>
                    <div class="flex items-center space-x-4 text-sm text-gray-500">
                        <span class="flex items-center space-x-1 hover:text-gray-300 transition-colors duration-200 cursor-help">
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M2.166 4.999A11.954 11.954 0 0010 1.944 11.954 11.954 0 0017.834 5c.11.65.166 1.32.166 2.001 0 5.225-3.34 9.67-8 11.317C5.34 16.67 2 12.225 2 7c0-.682.057-1.35.166-2.001zm11.541 3.708a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                            </svg>
                            <span>{"SSL Protected"}</span>
                        </span>
                        <span>{"|"}</span>
                        <span class="hover:text-gray-300 transition-colors duration-200 cursor-help">{"GDPR Compliant"}</span>
                    </div>
                </div>
            </div>
        </footer>
    }
}

/// Loading spinner component
#[function_component(LoadingSpinner)]
pub fn loading_spinner() -> Html {
    html! {
        <div class="flex flex-col justify-center items-center py-16 space-y-4 animate-fade-in">
            <div class="relative">
                <div class="animate-spin rounded-full h-16 w-16 border-4 border-primary-200"></div>
                <div class="absolute top-0 left-0 animate-spin rounded-full h-16 w-16 border-4 border-transparent border-t-primary-600"></div>
            </div>
            <p class="text-sm text-gray-500 font-medium animate-pulse">{"Loading verification portal..."}</p>
        </div>
    }
}
