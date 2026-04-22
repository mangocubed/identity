use leptos::either::{Either, EitherOf5};
use leptos::prelude::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;
use url::Url;
use uuid::Uuid;

use crate::pages::AuthenticatedPage;
use crate::presenters::ApplicationPresenter;
use crate::server_fns;

#[derive(Clone, Default, Params, PartialEq)]
struct AuthorizeQuery {
    client_id: Option<Uuid>,
    redirect_uri: Option<Url>,
    response_type: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
}

#[component]
pub fn AuthorizePage() -> impl IntoView {
    let query = use_query::<AuthorizeQuery>();
    let application_resource = Resource::new_blocking(
        move || query.get().unwrap_or_default().client_id,
        async move |id| {
            let Some(id) = id else {
                return Err(ServerFnError::Args("client_id is required".to_owned()));
            };

            server_fns::application(id).await
        },
    );
    let action = Action::new(move |query: &AuthorizeQuery| {
        let query = query.to_owned();
        async move {
            if query.response_type != Some("code".to_owned()) || query.code_challenge_method != Some("S256".to_owned())
            {
                return Err(ServerFnError::Args("Invalid arguments".to_owned()));
            }

            let (Some(application_id), Some(redirect_url), Some(code_challenge)) =
                (query.client_id, query.redirect_uri, query.code_challenge)
            else {
                return Err(ServerFnError::Args("Invalid arguments".to_owned()));
            };

            server_fns::create_authorization(application_id, redirect_url, code_challenge).await
        }
    });
    let action_value = action.value();

    Effect::watch(
        move || application_resource.get(),
        move |application, _, _| {
            if let Some(Ok(ApplicationPresenter {
                id: _,
                name: _,
                is_trusted: true,
            })) = application
            {
                action.dispatch(query.get_untracked().unwrap_or_default());
            }
        },
        false,
    );

    view! {
        <AuthenticatedPage title="Authorize Application">
            <Suspense>
                {move || Suspend::new(async move {
                    match (application_resource.get(), action_value.get()) {
                        (Some(Ok(ApplicationPresenter { id: _, name: _, is_trusted: true })), None)
                        | (Some(Ok(_)), Some(Ok(_))) => {
                            EitherOf5::A(view! { <div class="text-center">"Redirecting..."</div> })
                        }
                        (Some(Ok(_)), Some(Err(_))) => {
                            EitherOf5::B(view! { <div class="text-center">"Could not authorize application..."</div> })
                        }
                        (Some(Ok(application)), None) => {
                            EitherOf5::C(
                                view! {
                                    <div class="card card-border bg-base-100 max-w-160 mx-auto my-4 w-full">
                                        <div class="card-body">
                                            <p class="text-xl">
                                                "Authorize "<b>{application.name}</b>
                                                " to use your user account information"
                                            </p>

                                            <div class="card-actions">
                                                <button
                                                    on:click=move |_| {
                                                        action.dispatch(query.get_untracked().unwrap_or_default());
                                                    }
                                                    class="btn-submit"
                                                    disabled=move || action.pending().get()
                                                >
                                                    {move || {
                                                        if action.pending().get() {
                                                            Either::Left(
                                                                view! { <span class="loading loading-spinner" /> },
                                                            )
                                                        } else {
                                                            Either::Right("Authorize")
                                                        }
                                                    }}
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                },
                            )
                        }
                        (Some(Err(_)), _) => {
                            EitherOf5::D(view! { <div class="text-center">"Application not found"</div> })
                        }
                        (_, _) => EitherOf5::E(()),
                    }
                })}
            </Suspense>
        </AuthenticatedPage>
    }
}
