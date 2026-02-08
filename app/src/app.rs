use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::StaticSegment;

use crate::components::Mango3Logo;
use crate::icons::Mango3Icon;
use crate::pages::HomePage;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos_meta::MetaTags;

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0, viewport-fit: contain"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
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

    let brand_dev = || {
        if cfg!(debug_assertions) {
            Either::Left(view! { <div class="brand-dev">"(dev)"</div> })
        } else {
            Either::Right(())
        }
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/application.css" />

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
                </div>

                <div class="layout">
                    <main class="main">
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route path=StaticSegment("") view=HomePage />
                        </Routes>
                    </main>

                    <footer class="footer">
                        <aside class="opacity-75">
                            <p>{format!("Version: {}", env!("CARGO_PKG_VERSION"))}</p>
                        </aside>
                    </footer>
                </div>
            </Router>
        </div>
    }
}
