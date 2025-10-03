use dioxus::prelude::*;

use sdk::components::{Brand, Footer, Navbar, NavbarStart};
use sdk::constants::{COPYRIGHT, PRIVACY_URL, TERMS_URL};

use crate::constants::SOURCE_CODE_URL;
use crate::routes::Routes;

#[component]
pub fn GuestLayout() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Navbar {
                NavbarStart {
                    Link { to: Routes::home(),
                        Brand { "ID" }
                    }
                }
            }

            main { class: "main grow", Outlet::<Routes> {} }

            Footer {
                aside { class: "opacity-75",
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

                    p { {COPYRIGHT} }
                }

                nav {
                    a { class: "link", href: TERMS_URL, target: "_blank", "Terms of Service" }

                    a { class: "link", href: PRIVACY_URL, target: "_blank", "Privacy Policy" }

                    a {
                        class: "link",
                        href: SOURCE_CODE_URL.clone(),
                        target: "_blank",
                        "Source code"
                    }
                }
            }
        }
    }
}

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
