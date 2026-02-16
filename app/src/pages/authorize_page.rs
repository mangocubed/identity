use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;
use url::Url;
use uuid::Uuid;

use crate::pages::AuthenticatedPage;
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
    let authorize_resource = Resource::new_blocking(
        move || query.get().unwrap_or_default(),
        async move |query| {
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
        },
    );

    view! {
        <AuthenticatedPage title="Authorize Application">
            <Suspense>
                {move || Suspend::new(async move {
                    match authorize_resource.get() {
                        Some(Ok(_)) => {
                            EitherOf3::A(view! { <div class="text-center">"Redirecting..."</div> })
                        }
                        Some(Err(_)) => {
                            EitherOf3::B(
                                view! {
                                    <div class="text-center">
                                        "Could not authorize application..."
                                    </div>
                                },
                            )
                        }
                        None => {
                            EitherOf3::C(
                                view! {
                                    <div class="text-center">"Authorizing application..."</div>
                                },
                            )
                        }
                    }
                })}
            </Suspense>
        </AuthenticatedPage>
    }
}
