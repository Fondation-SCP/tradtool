use crate::main_component::Main;
use leptos::prelude::*;

mod main_component;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <Main />
        }
    })
}
