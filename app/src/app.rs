use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{StaticSegment, path};

use crate::components::{Alert, Navbar};
use crate::hooks::{provide_current_user_resource, provide_toast};
use crate::pages::{AuthorizePage, HomePage, LoginPage, RegisterPage};
use crate::utils::sleep;

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
    #[cfg(feature = "ssr")]
    {
        leptos::task::spawn(async {
            use identity_core::commands;

            use crate::server_fns;

            if let Ok(session) = server_fns::extract_session().await
                && session.should_refresh()
            {
                let _ = commands::refresh_session(&session).await;
            }
        });
    }

    provide_meta_context();
    provide_current_user_resource();

    let mut toast = provide_toast();

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
                <Navbar />

                <div class="layout">
                    <main class="main">
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route path=StaticSegment("") view=HomePage />
                            <Route path=path!("/oauth/authorize") view=AuthorizePage />
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
                            sleep(5000).await;
                            toast.remove_alert(id);
                        });
                    });

                    view! { <Alert alert_type=alert_type>{message}</Alert> }
                }
            />
        </div>
    }
}
