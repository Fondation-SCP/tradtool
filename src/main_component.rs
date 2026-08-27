use kode_leptos::CodeEditor;
use leptos::prelude::*;

#[component]
pub(crate) fn Main() -> impl IntoView {
    view! {
        <main style="display: flex; height: 100vh;">
            <CodeEditor />
            <CodeEditor />
        </main>
    }
}
