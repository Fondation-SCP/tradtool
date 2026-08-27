use kode_leptos::CodeEditor;
use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "main.module.scss");

#[component]
pub(crate) fn Main() -> impl IntoView {
    view! {
        <main class=style::main>
            <CodeEditor />
            <CodeEditor />
        </main>
    }
}
