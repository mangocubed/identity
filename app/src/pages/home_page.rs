use leptos::prelude::*;

use crate::pages::AuthenticatedPage;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <AuthenticatedPage title="Home">
            <div>{"Welcome to Mango3!"}</div>
        </AuthenticatedPage>
    }
}
