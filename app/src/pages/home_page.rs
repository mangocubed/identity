use dioxus::prelude::*;
use sdk::components::PageTitle;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        PageTitle { "Home" }
    }
}
