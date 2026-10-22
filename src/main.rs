use crate::main_component::Main;
use leptos::prelude::*;
use stylance::import_style;

mod main_component;

import_style!(style, "layout.module.scss");

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <div class=style::layout>
                <Main />
            </div>
        }
    })
}
