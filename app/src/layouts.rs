use dioxus::prelude::*;

use sdk::app::components::*;
use sdk::app::icons::{ChevronDownMini, EnvelopeOutline, HomeOutline, InformationCircleOutline, PasswordOutline};
use sdk::app::run_with_spinner;
use sdk::constants::{COPYRIGHT, PRIVACY_URL, TERMS_URL};

use crate::components::AboutModal;
use crate::constants::SOURCE_CODE_URL;
use crate::hooks::use_current_user;
use crate::routes::Routes;
use crate::server_fns;
use crate::storage::{delete_redirect_to, delete_session, get_redirect_to, set_redirect_to};

#[component]
pub fn LoginLayout() -> Element {
    let navigator = use_navigator();
    let current_user = use_current_user();

    use_effect(move || {
        if let Some(Ok(_)) = &*current_user.read() {
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
    let mut show_about = use_signal(|| false);
    let mut show_logout_confirmation = use_signal(|| false);

    use_effect(move || {
        if let Some(Err(_)) = *current_user.read() {
            set_redirect_to(&router.full_route_string());
            navigator.push(Routes::login());
        }
    });

    rsx! {
        if let Some(Ok(user)) = &*current_user.read() {
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
                                let _ = run_with_spinner("logout", server_fns::logout).await;

                                delete_session();
                                current_user.restart();
                            }
                        },
                        "Are you sure you want to logout?"
                    }
                }
            }

            div { class: "flex m-4 min-h-[calc(100vh-6rem)]",
                div { class: "shrink-0 bg-base-200 rounded-box md:w-56 flex flex-col items-between",
                    ul { class: "menu md:w-56",
                        li {
                            class: "max-md:tooltip max-md:tooltip-right",
                            "data-tip": "Home",
                            Link { to: Routes::home(),
                                HomeOutline {}

                                span { class: "max-md:hidden", "Home" }
                            }
                        }

                        div { class: "divider m-1" }

                        li {
                            class: "max-md:tooltip max-md:tooltip-right",
                            "data-tip": "Email",
                            Link { to: Routes::email(),
                                EnvelopeOutline {}

                                span { class: "max-md:hidden", "Email" }
                            }
                        }

                        li {
                            class: "max-md:tooltip max-md:tooltip-right",
                            "data-tip": "Change password",
                            Link { to: Routes::change_password(),
                                PasswordOutline {}

                                span { class: "max-md:hidden", "Change password" }
                            }
                        }
                    }

                    ul { class: "menu md:w-56 mt-auto",
                        li {
                            class: "max-md:tooltip max-md:tooltip-right",
                            "data-tip": "About",
                            a {
                                onclick: move |_| {
                                    *show_about.write() = true;
                                },
                                InformationCircleOutline {}

                                span { class: "max-md:hidden", "About" }
                            }
                        }
                    }
                }

                main { class: "main grow max-w-[calc(100%-48px)] md:max-w-[calc(100%-208px)]",
                    Outlet::<Routes> {}
                }

                AboutModal { is_open: show_about }
            }
        }
    }
}
