use dioxus::prelude::*;

use sdk::app::components::{H1, PageTitle};

#[component]
pub fn HomePage() -> Element {
    rsx! {
        PageTitle { "Home" }

        H1 { "Home" }
    }
}
