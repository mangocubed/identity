use dioxus::prelude::*;

use sdk::app::components::{Brand, Modal};
use sdk::constants::{COPYRIGHT, PRIVACY_URL, TERMS_URL};

use crate::constants::SOURCE_CODE_URL;

#[component]
pub fn AboutModal(is_open: Signal<bool>) -> Element {
    rsx! {
        Modal { is_open, class: "gap-4 flex flex-col items-center",
            Brand { "ID" }

            div { class: "text-center text-sm opacity-75",
                p {
                    "Version: "
                    {env!("CARGO_PKG_VERSION")}
                    " ("
                    {env!("GIT_REV_SHORT")}
                    ")"
                }

                p {
                    "Built on: "
                    {env!("BUILD_DATETIME")}
                }
            }

            div {
                a { class: "link", href: TERMS_URL, target: "_blank", "Terms of Service" }

                span { class: "opacity-50", " | " }

                a { class: "link", href: PRIVACY_URL, target: "_blank", "Privacy Policy" }

                span { class: "opacity-50", " | " }

                a {
                    class: "link",
                    href: SOURCE_CODE_URL.clone(),
                    target: "_blank",
                    "Source code"
                }
            }

            div { class: "opacity-75", {COPYRIGHT} }
        }
    }
}
