use leptos::{ev::MouseEvent, prelude::*};

/// Variantes del botón
#[derive(Default, Clone, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => {
                "bg-destructive text-destructive-foreground hover:bg-destructive/90"
            }
            ButtonVariant::Outline => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            }
            ButtonVariant::Secondary => {
                "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

/// Tamaños del botón
#[derive(Default, Clone, PartialEq)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

impl ButtonSize {
    fn class(&self) -> &'static str {
        match self {
            ButtonSize::Default => "h-10 px-4 py-2",
            ButtonSize::Sm => "h-9 rounded-md px-3",
            ButtonSize::Lg => "h-11 rounded-md px-8",
            ButtonSize::Icon => "h-10 w-10",
        }
    }
}

#[component]
pub fn Button(
    // Props personalizados
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,

    // Global attributes
    #[prop(into, optional)] autofocus: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,

    // Attributes from <button>
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] formaction: MaybeProp<String>,
    #[prop(into, optional)] formenctype: MaybeProp<String>,
    #[prop(into, optional)] formmethod: MaybeProp<String>,
    #[prop(into, optional)] formnovalidate: Signal<bool>,
    #[prop(into, optional)] formtarget: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] r#type: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,

    // Event handlers
    #[prop(into, optional)] onclick: Option<Callback<MouseEvent>>,

    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = "inline-flex items-center justify-center gap-2 whitespace-nowrap \
        rounded-md text-sm font-medium ring-offset-background transition-colors \
        focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring \
        focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 \
        [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0";

    let classes = format!(
        "{} {} {} {}",
        base_class,
        variant.class(),
        size.class(),
        class.get().unwrap_or_default()
    );

    view! {
        <button
            class=classes
            id=id
            autofocus=autofocus
            disabled=disabled
            form=form
            formaction=formaction
            formenctype=formenctype
            formmethod=formmethod
            formnovalidate=formnovalidate
            formtarget=formtarget
            name=name
            r#type=r#type
            value=value
            on:click=move |ev| if let Some(cb) = onclick.as_ref() { cb.run(ev) }
        >
            {children.map(|c| c())}
        </button>
    }
}
