use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::utils::sleep;

#[component]
pub fn ConfirmationModal(
    children: ChildrenFn,
    is_open: RwSignal<bool>,
    #[prop(into)] on_accept: Callback<()>,
) -> impl IntoView {
    let children_store = StoredValue::new(children);

    view! {
        <Modal is_closable=false is_open=is_open>
            <div>{children_store.read_value()()}</div>

            <div class="modal-action">
                <button
                    class="btn"
                    on:click=move |event| {
                        event.prevent_default();
                        *is_open.write() = false;
                    }
                >
                    "Cancel"
                </button>

                <button
                    class="btn btn-primary"
                    on:click=move |event| {
                        event.prevent_default();
                        *is_open.write() = false;
                        on_accept.run(());
                    }
                >
                    "Accept"
                </button>
            </div>
        </Modal>
    }
}

#[component]
pub fn Modal(
    children: ChildrenFn,
    is_open: RwSignal<bool>,
    #[prop(default = true)] is_closable: bool,
    #[prop(into, optional)] on_open: Option<Callback<()>>,
    #[prop(into, optional)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let is_visible = RwSignal::new(*is_open.read_untracked());
    let children_store = StoredValue::new(children);

    Effect::new(move |_| {
        if !*is_open.read() && *is_visible.read() {
            if let Some(on_close) = on_close {
                on_close.run(());
            }

            spawn_local(async move {
                sleep(300).await;
                is_visible.set(false);
            });
        }
    });
    view! {
        <Show when=move || {
            *is_open.read() || *is_visible.read()
        }>
            {move || {
                Effect::new(move |_| {
                    spawn_local(async move {
                        sleep(5).await;
                        is_visible.set(true);
                        if let Some(on_open) = on_open {
                            sleep(300).await;
                            on_open.run(());
                        }
                    })
                });

                view! {
                    <dialog class="modal" class:modal-open=move || *is_open.read() && *is_visible.read()>
                        {if is_closable {
                            Some(
                                view! {
                                    <button class="modal-close" on:click=move |_| is_open.set(false)>
                                        "✕"
                                    </button>
                                },
                            )
                        } else {
                            None
                        }}

                        <div class="modal-box">{children_store.read_value()()}</div>

                        {if is_closable {
                            Some(view! { <div class="modal-backdrop" on:click=move |_| is_open.set(false) /> })
                        } else {
                            None
                        }}
                    </dialog>
                }
            }}
        </Show>
    }
}
