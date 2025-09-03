use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=format!("rounded-lg border bg-white text-black shadow-sm{}", class.map(|c| format!(" {c}")).unwrap_or_default())
            id=id
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <header
            class=format!("flex flex-col gap-1.5 p-6{}", class.map(|c| format!(" {c}")).unwrap_or_default())
            id=id
        >
            {children()}
        </header>
    }
}

#[component]
pub fn CardTitle(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <h2
            class=format!("text-2xl font-semibold leading-none tracking-tight{}", class.map(|c| format!(" {c}")).unwrap_or_default())
            id=id
        >
            {children()}
        </h2>
    }
}

#[component]
pub fn CardDescription(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <p
            class=format!("text-sm text-gray-500{}", class.map(|c| format!(" {c}")).unwrap_or_default())
            id=id
        >
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            class=format!("p-6 pt-0{}", class.map(|c| format!(" {c}")).unwrap_or_default())
            id=id
        >
            {children()}
        </section>
    }
}

#[component]
pub fn CardFooter(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <footer
            class=format!("flex items-center p-6 pt-0{}", class.map(|c| format!(" {c}")).unwrap_or_default())
            id=id
        >
            {children()}
        </footer>
    }
}
