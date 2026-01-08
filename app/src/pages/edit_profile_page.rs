use dioxus::prelude::*;

use sdk::app::components::{Form, FormSuccessModal, H1, PageTitle, SelectField, TextField};
use sdk::app::hooks::{use_form_provider, use_resource_with_spinner};

use crate::routes::Routes;
use crate::server_fns;

#[component]
pub fn EditProfilePage() -> Element {
    use_form_provider("update-profile", server_fns::update_profile);

    let user_profile_resource = use_resource_with_spinner("user-profile", server_fns::current_user_profile);
    let mut full_name = use_signal(String::new);
    let mut birthdate = use_signal(String::new);
    let mut country_alpha2 = use_signal(String::new);

    let navigator = use_navigator();

    use_effect(move || {
        if let Some(Ok(user_profile)) = &*user_profile_resource.read() {
            full_name.set(user_profile.full_name.clone());
            birthdate.set(user_profile.birthdate.to_string());
            country_alpha2.set(user_profile.country_alpha2.clone());
        }
    });

    rsx! {
        PageTitle { "Edit Profile" }

        H1 { "Edit Profile" }

        FormSuccessModal {
            on_close: move |_| {
                navigator.push(Routes::home());
            },
        }

        Form {
            TextField {
                id: "full_name",
                label: "Full name",
                name: "full_name",
                value: full_name,
            }

            TextField {
                id: "birthdate",
                label: "Birthdate",
                name: "birthdate",
                input_type: "date",
                value: birthdate,
            }

            SelectField {
                id: "country_alpha2",
                label: "Country",
                name: "country_alpha2",
                option { "Select" }
                for country in rust_iso3166::ALL {
                    option {
                        key: "{country.alpha2}",
                        value: country.alpha2,
                        selected: country.alpha2 == country_alpha2(),
                        {country.name}
                    }
                }
            }
        }
    }
}
