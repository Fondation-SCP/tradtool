use kode_leptos::{CodeEditor, Theme};
use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "main.module.scss");

#[component]
pub(crate) fn Main(theme: Theme) -> impl IntoView {
    view! {
        <main class=style::main>
            <CodeEditor theme=theme.clone() />
            <CodeEditor theme=theme.clone() />
        </main>
    }
}
