use leptos::prelude::*;

use crate::hooks::use_current_user_resource;
use crate::presenters::UserPresenter;
use crate::server_fns::ServerFnResult;

#[component]
pub fn CurrentUser<VF, IV>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(ServerFnResult<UserPresenter>) -> IV + Send + Sync + 'static,
{
    let current_user_resource = use_current_user_resource();
    let children_store = StoredValue::new(children);

    view! {
        <Transition>
            {move || Suspend::new(async move {
                current_user_resource.get().map(|user| children_store.with_value(|store| store(user)))
            })}
        </Transition>
    }
}
