use dioxus::prelude::*;
use sdk::hooks::use_resource_with_loader;
use uuid::Uuid;

use sdk::components::{H1, PageTitle};

use crate::server_fns::attempt_to_authorize;

#[component]
pub fn AuthorizePage(client_id: Uuid) -> Element {
    let navigator = use_navigator();
    let authorize = use_resource_with_loader("authorize", move || attempt_to_authorize(client_id));

    rsx! {
        match authorize() {
            Some(Ok(url)) => {
                let _ = navigator.push(url.to_string());

                rsx! {
                    PageTitle { "Redirecting..." }

                    H1 { "Redirecting..." }
                }
            }
            Some(Err(_)) => {
                rsx! {
                    PageTitle { "Could not authorize application" }

                    H1 { "Could not authorize application" }
                }
            }
            None => {
                rsx! {
                    PageTitle { "Authorizing application..." }

                    H1 { "Authorizing application..." }
                }
            }
        }
    }
}
