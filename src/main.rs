use crate::main_component::Main;
use kode_leptos::Theme;
use leptos::prelude::*;
use stylance::{classes, import_style};

mod main_component;

import_style!(style, "layout.module.scss");

fn main() {
    leptos::mount::mount_to_body(|| {
        let theme = Theme::dark();
        let (sidebar_collapsed, set_sidebar_collapsed) = signal(true);

        view! {
            <div style=theme.to_css_vars() class=style::layout>
                <nav class=move || {
                    classes!(style::sidebar, sidebar_collapsed.get().then_some(style::collapsed))
                }>
                    <button on:click=move |_| {
                        set_sidebar_collapsed.update(|c| *c = !*c)
                    }>"😎"</button>
                    <ul class=style::nav_links>
                        <li>
                            <span>"Menu item"</span>
                        </li>
                    </ul>
                </nav>
                <Main theme=theme.clone() />
            </div>
        }
    })
}
