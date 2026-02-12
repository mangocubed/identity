use std::time::Duration;

use leptos::either::Either;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{Title, provide_meta_context};
use leptos_router::StaticSegment;
use leptos_router::components::{A, Route, Router, Routes};

use crate::components::{Alert, Mango3Logo};
use crate::hooks::{provide_current_user_resource, provide_toast};
use crate::icons::Mango3Icon;
use crate::pages::{HomePage, LoginPage, RegisterPage};

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos_meta::{HashedStylesheet, MetaTags};

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0, viewport-fit: contain"
                />
                <meta name="robots" content="noindex, nofollow" />
                <link rel="icon" href="/favicon.ico" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags />
                <HashedStylesheet id="leptos" options=options />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_current_user_resource();

    let mut toast = provide_toast();

    let brand_dev = || {
        if cfg!(debug_assertions) {
            Either::Left(view! { <div class="brand-dev">"(dev)"</div> })
        } else {
            Either::Right(())
        }
    };

    view! {
        <Title formatter=|page_title: String| {
            format!(
                "{}Mango³ ID{}",
                if !page_title.is_empty() { format!("{page_title} | ") } else { String::new() },
                if cfg!(debug_assertions) { " (dev)" } else { "" },
            )
        } />

        <div class="wrapper">
            <Router>
                <div class="navbar">
                    <div class="navbar-start">
                        <A href="/">
                            <div class="brand">
                                <Mango3Icon class="brand-icon" />

                                <Mango3Logo class="brand-logo" />

                                <div class="brand-suffix">"ID"</div>

                                {brand_dev}
                            </div>
                        </A>
                    </div>

                    <div class="navbar-end">
                        <A attr:class="btn btn-outline" href="/login">
                            "Login"
                        </A>
                    </div>
                </div>

                <div class="layout">
                    <main class="main">
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route path=StaticSegment("") view=HomePage />
                            <Route path=StaticSegment("login") view=LoginPage />
                            <Route path=StaticSegment("register") view=RegisterPage />
                        </Routes>
                    </main>

                    <footer class="footer">
                        <aside class="opacity-75">
                            <p>{format!("Version: {}", env!("CARGO_PKG_VERSION"))}</p>

                            <p>"© 2026 Mango³ Group"</p>
                        </aside>
                    </footer>
                </div>
            </Router>
        </div>

        <div class="toast">
            <For
                each=move || toast.alerts()
                key=|(id, _, _)| *id
                children=move |(id, alert_type, message)| {
                    Effect::new(move |_| {
                        spawn_local(async move {
                            gloo_timers::future::sleep(Duration::from_millis(5000)).await;
                            toast.remove_alert(id);
                        });
                    });

                    view! { <Alert alert_type=alert_type>{message}</Alert> }
                }
            />
        </div>
    }
}
