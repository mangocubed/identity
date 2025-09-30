use dioxus::prelude::*;

use sdk::components::{Brand, Navbar, NavbarStart};

use crate::routes::Routes;

#[component]
pub fn UserLayout() -> Element {
    rsx! {
        Navbar {
            NavbarStart {
                Link { to: Routes::home(),
                    Brand { "ID" }
                }
            }
        }

        Outlet::<Routes> {}
    }
}
