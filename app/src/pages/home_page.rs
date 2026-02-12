use leptos::prelude::*;

use crate::pages::AuthenticatedPage;

#[component]
pub fn HomePage() -> impl IntoView {
    view! { <AuthenticatedPage title="Home">{"Welcome to Mango3!"}</AuthenticatedPage> }
}
