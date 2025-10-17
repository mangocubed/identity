use dioxus::prelude::*;

use sdk::app::components::*;
use sdk::app::icons::ChevronDownMini;
use sdk::app::run_with_spinner;
use sdk::constants::{COPYRIGHT, PRIVACY_URL, TERMS_URL};

use crate::constants::SOURCE_CODE_URL;
use crate::hooks::use_current_user;
use crate::local_data::{delete_redirect_to, get_redirect_to};
use crate::local_data::{delete_session_token, set_redirect_to};
use crate::requests;
use crate::routes::Routes;

#[component]
pub fn LoginLayout() -> Element {
    let navigator = use_navigator();
    let current_user = use_current_user();

    use_effect(move || {
        if let Some(Some(_)) = &*current_user.read() {
            navigator.push(get_redirect_to());
            delete_redirect_to();
        }
    });

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
    let navigator = use_navigator();
    let router = router();
    let mut current_user = use_current_user();
    let mut show_logout_confirmation = use_signal(|| false);

    use_effect(move || {
        if let Some(None) = *current_user.read() {
            set_redirect_to(&router.full_route_string());
            navigator.push(Routes::login());
        }
    });

    rsx! {
        if let Some(Some(user)) = &*current_user.read() {
            Navbar {
                NavbarStart {
                    Link { to: Routes::home(),
                        Brand { "ID" }
                    }
                }

                NavbarEnd {
                    Dropdown { class: "dropdown-end",
                        div { class: "flex gap-2 items-center", tabindex: 0,
                            div { class: "text-left text-sm",
                                div { class: "mb-1 font-bold", {user.display_name.clone()} }
                                div { class: "opacity-70",
                                    "@"
                                    {user.username.clone()}
                                }
                            }

                            ChevronDownMini {}
                        }

                        DropdownContent { class: "mt-4",
                            ul { class: "menu p-2", tabindex: 0,
                                li {
                                    a {
                                        onclick: move |_| {
                                            *show_logout_confirmation.write() = true;
                                        },
                                        "Logout"
                                    }
                                }
                            }
                        }
                    }

                    ConfirmationModal {
                        is_open: show_logout_confirmation,
                        on_accept: move |()| {
                            async move {
                                let _ = run_with_spinner("logout", requests::logout).await;

                                delete_session_token();
                                current_user.restart();

                            }
                        },
                        "Are you sure you want to logout?"
                    }
                }
            }
        }

        main { class: "main grow", Outlet::<Routes> {} }
    }
}
