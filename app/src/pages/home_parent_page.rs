use leptos::prelude::*;
use leptos_router::components::{A, Outlet};

use crate::icons::{HomeOutline, PasswordOutline};

#[component]
pub fn HomeParentPage() -> impl IntoView {
    view! {
        <div class="flex gap-3 h-full">
            <div class="sidebar">
                <ul>
                    <li data-tip="Home">
                        <A href="/">
                            <HomeOutline />

                            <span>"Home"</span>
                        </A>
                    </li>

                    <li data-tip="Change password">
                        <A href="/change-password">
                            <PasswordOutline />

                            <span>"Change password"</span>
                        </A>
                    </li>
                </ul>
            </div>

            <div class="w-full">
                <Outlet />
            </div>
        </div>
    }
}
