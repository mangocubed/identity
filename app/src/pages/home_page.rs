use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Home" />

        <h1 class="h1">"Home"</h1>
    }
}
