use std::cell::LazyCell;
use std::collections::HashMap;

use leptos::prelude::*;

const ELEMENTS: LazyCell<HashMap<&str, &str>> = LazyCell::new(|| {
    HashMap::from_iter([
        ("about", "Acerca de"),
        ("open-source", "Open Source"),
        ("projects", "Proyectos"),
        ("consulting", "Consultoría"),
        ("talks", "Charlas"),
        ("community", "Comunidad"),
        ("contact", "Contacto"),
    ])
});

#[component]
fn NavigatorButton(#[prop(into)] id: String, #[prop(into)] name: String) -> impl IntoView {
    view! {
        <a
            class="block px-6 py-4 text-lg font-medium hover:bg-gray-100 rounded-md transition-colors duration-200"
            href=format!("#{id}")
        >
            {name}
        </a>
    }
}

#[component]
pub fn Navigator() -> impl IntoView {
    view! {
        <nav class="fixed px-4 md:px-0 pt-4 w-full top-0 left-0 flex flex-row items-center justify-end md:justify-center">
            // --- Hidden Checkbox ---
            <input id="menu-toggle" type="checkbox" class="peer/menu-toggle hidden"/>
            <div class="flex items-center justify-center bg-white border-b border-gray-200 shadow-sm px-0 py-0 md:px-4 md:py-3 md:rounded-md md:w-auto md:mx-auto md:mt-4">
                // --- Hamburger Buton ---
                <label for="menu-toggle" class="md:hidden cursor-pointer px-6 py-6 rounded-md hover:bg-gray-100 transition-colors duration-200">
                    <div class="space-y-1">
                        <span class="block w-6 h-0.5 bg-black"></span>
                        <span class="block w-6 h-0.5 bg-black"></span>
                        <span class="block w-6 h-0.5 bg-black"></span>
                    </div>
                </label>

                // --- Desktop ---
                <div class="hidden md:flex md:flex-row">
                    {ELEMENTS.iter().map(|(id, name)| {
                        view! { <NavigatorButton id=*id name=*name /> }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            // --- Mobile Menu ---
            <div
                class="
                    fixed inset-0 z-40
                    flex flex-col items-center justify-center bg-white
                    translate-x-full transition-transform duration-300 ease-in-out
                    peer-checked/menu-toggle:translate-x-0
                    md:hidden
                "
            >
                <label for="menu-toggle" class="absolute top-4 right-4 cursor-pointer px-6 py-4 rounded-md hover:bg-gray-100">
                    {"✕"}
                </label>

                {ELEMENTS.iter().map(|(id, name)| {
                    view! { <NavigatorButton id=*id name=*name /> }
                }).collect::<Vec<_>>()}
            </div>
        </nav>
    }
}
