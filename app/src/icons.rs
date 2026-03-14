use leptos::prelude::*;

#[component]
pub fn CheckCircleOutline<'a>(#[prop(default = "size-6")] class: &'a str) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class=class
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
            />
        </svg>
    }
}

#[component]
pub fn ChevronDownMini<'a>(#[prop(default = "size-5")] class: &'a str) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class=class>
            <path
                fill-rule="evenodd"
                d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z"
                clip-rule="evenodd"
            />
        </svg>
    }
}

#[component]
pub fn ExclamationOutline<'a>(#[prop(default = "size-6")] class: &'a str) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class=class
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"
            />
        </svg>
    }
}

#[component]
pub fn EyeMini<'a>(#[prop(default = "size-5")] class: &'a str) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class=class>
            <path d="M10 12.5a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z" />
            <path
                fill-rule="evenodd"
                d="M.664 10.59a1.651 1.651 0 0 1 0-1.186A10.004 10.004 0 0 1 10 3c4.257 0 7.893 2.66 9.336 6.41.147.381.146.804 0 1.186A10.004 10.004 0 0 1 10 17c-4.257 0-7.893-2.66-9.336-6.41ZM14 10a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z"
                clip-rule="evenodd"
            />
        </svg>
    }
}

#[component]
pub fn EyeSlashMini<'a>(#[prop(default = "size-5")] class: &'a str) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class=class>
            <path
                fill-rule="evenodd"
                d="M3.28 2.22a.75.75 0 0 0-1.06 1.06l14.5 14.5a.75.75 0 1 0 1.06-1.06l-1.745-1.745a10.029 10.029 0 0 0 3.3-4.38 1.651 1.651 0 0 0 0-1.185A10.004 10.004 0 0 0 9.999 3a9.956 9.956 0 0 0-4.744 1.194L3.28 2.22ZM7.752 6.69l1.092 1.092a2.5 2.5 0 0 1 3.374 3.373l1.091 1.092a4 4 0 0 0-5.557-5.557Z"
                clip-rule="evenodd"
            />
            <path d="m10.748 13.93 2.523 2.523a9.987 9.987 0 0 1-3.27.547c-4.258 0-7.894-2.66-9.337-6.41a1.651 1.651 0 0 1 0-1.186A10.007 10.007 0 0 1 2.839 6.02L6.07 9.252a4 4 0 0 0 4.678 4.678Z" />
        </svg>
    }
}

#[component]
pub fn InformationCircleOutline<'a>(#[prop(default = "size-6")] class: &'a str) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class=class
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z"
            />
        </svg>
    }
}

#[component]
pub fn Mango3Icon<'a>(#[prop(optional)] class: &'a str) -> impl IntoView {
    view! {
        <svg class=class width="124" height="124" viewBox="0 0 124 124" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M62.1326 110C55.1492 110 48.6961 108.81 42.7735 106.43C36.9392 104.05 31.8563 100.744 27.5249 96.5124C23.2818 92.1929 19.9668 87.124 17.5801 81.3058C15.1934 75.3995 14 68.9642 14 62C14 55.124 15.1934 48.7769 17.5801 42.9587C19.9668 37.0523 23.2818 31.9394 27.5249 27.6198C31.8563 23.3003 36.9392 19.9504 42.7735 17.5703C48.6961 15.1901 55.1492 14 62.1326 14C69.0276 14 75.3923 15.1901 81.2265 17.5703C87.0608 19.9504 92.0994 23.3003 96.3425 27.6198C100.674 31.9394 104.033 37.0523 106.42 42.9587C108.807 48.7769 110 55.168 110 62.1322C110 69.0083 108.807 75.3554 106.42 81.1736C104.033 86.9917 100.674 92.0606 96.3425 96.3802C92.0994 100.7 87.0608 104.05 81.2265 106.43C75.3923 108.81 69.0276 110 62.1326 110Z"
                fill="#FF4C00"
            />
            <path
                d="M113.778 28.9333C111.739 28.9333 109.923 28.7281 108.332 28.3177C106.74 27.856 105.074 27.1891 103.333 26.317L106.69 20.2379C107.586 20.7509 108.456 21.1613 109.302 21.4691C110.197 21.7256 111.117 21.8539 112.062 21.8539C113.157 21.8539 114.052 21.5974 114.748 21.0844C115.445 20.5201 115.793 19.8019 115.793 18.9298C115.793 18.1603 115.469 17.5447 114.823 17.083C114.226 16.57 113.355 16.3135 112.211 16.3135C111.565 16.3135 110.893 16.3904 110.197 16.5443C109.501 16.6982 108.804 16.9034 108.108 17.1599L106.392 12.3121L116.837 4.92482L117.434 7.00248H105.422V0H121.612L122.358 6.46383L113.554 13.0046L113.107 12.466C113.604 12.1582 114.151 11.9273 114.748 11.7734C115.345 11.6195 115.818 11.5426 116.166 11.5426C118.454 11.5426 120.319 12.2864 121.761 13.7741C123.254 15.2105 124 17.1343 124 19.5454C124 21.3922 123.527 23.0338 122.582 24.4702C121.637 25.8553 120.394 26.9583 118.852 27.7791C117.31 28.5486 115.619 28.9333 113.778 28.9333Z"
                fill="#FF4C00"
            />
        </svg>
    }
}
