use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};

#[cfg(debug_assertions)]
const ASSETS_FOLDER: &str = "./assets";

#[cfg(not(debug_assertions))]
const ASSTES_FOLDERS: &str = ".";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                <meta name="theme-color" media="(prefers-color-scheme: light)" content="#fed7aa"/>
                <meta name="theme-color" media="(prefers-color-scheme: dark)" content="#181811"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
                <Meta charset="utf-8"/>
                <Meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Stylesheet id="fonts" href=format!("{ASSETS_FOLDER}/fonts.css")/>
                <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
                <Title text="Sergio Ribera"/>
                <Meta
                    name="description"
                    content="Desarrollador Rust especializado en sistemas de alto rendimiento y consultor técnico"
                />

                <Meta property="og:site_name" content="Conoce a Sergio Ribera"/>
                <Meta property="og:title" content="Conoce a Sergio Ribera"/>
                <Meta
                    property="og:description"
                    content="Desarrollador Rust especializado en sistemas de alto rendimiento y consultor técnico"
                />
                <Meta property="og:url" content="https://sergioribera.rs"/>

                <Meta property="twitter:card" content="summary_large_image"/>
                <Meta property="twitter:site" content="@sergioribera_rs"/>

                <Link rel="canonical" href="https://sergioribera.rs"/>
                <Meta property="og:image" content="https://sergioribera.rs/og.png"/>
                <Meta property="twitter:image" content="https://sergioribera.rs/og.png"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <main>
        </main>
    }
}
