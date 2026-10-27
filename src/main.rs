mod data;
mod pages;
mod ui;
use leptos::prelude::*;
use ui::*;
use wasm_bindgen::{JsCast, closure::Closure};
fn current_route() -> String {
    window()
        .location()
        .hash()
        .unwrap_or_default()
        .trim_start_matches('#')
        .trim_matches('/')
        .to_string()
}
fn main() {
    console_error_panic_hook::set_once();
    if let Some(boot) = document().get_element_by_id("boot") {
        boot.remove();
    }
    leptos::mount::mount_to_body(App);
}
#[component]
fn App() -> impl IntoView {
    let route = RwSignal::new(current_route());
    let prefers_reduced = window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false);
    let reduced = RwSignal::new(prefers_reduced);
    let listener = Closure::<dyn FnMut()>::new(move || route.set(current_route()));
    window()
        .add_event_listener_with_callback("hashchange", listener.as_ref().unchecked_ref())
        .expect("hash navigation");
    listener.forget();
    Effect::new(move |_| {
        let r = route.get();
        let title = if let Some(id) = r.strip_prefix("domain/") {
            data::domain(id).name.clone()
        } else {
            match r.as_str() {
                "atlas" => "Ecosystem atlas",
                "systems" => "Operating domains",
                "flows" => "Production flows",
                "architecture" => "Connected architecture",
                "governance" => "Authority & rights",
                "roadmap" => "Path to independence",
                _ => "The Big One",
            }
            .to_string()
        };
        document().set_title(&format!("{title} — Saber Tech"));
    });
    view! {
        <div class="app" class:reduced=move || reduced.get() class:landing=move || route.get().is_empty()>
            <a class="skip-link" href="#main-content" on:click=move |ev|{ev.prevent_default();if let Some(el)=document().get_element_by_id("main-content"){if let Ok(el)=el.dyn_into::<web_sys::HtmlElement>(){let _=el.focus();}}}>"Skip to content"</a>
            <Header route=route reduced=reduced />
            <main id="main-content" tabindex="-1">
                {move || {
                    let r=route.get();
                    match r.as_str() {
                        "" => pages::Landing().into_any(), "atlas" => pages::Atlas().into_any(), "systems" => pages::Systems().into_any(), "flows" => pages::Flows().into_any(), "architecture" => pages::Architecture().into_any(), "governance" => pages::Governance().into_any(), "roadmap" => pages::Roadmap().into_any(),
                        _ if r.starts_with("domain/") => view! { <pages::DomainPage id=r[7..].to_string()/> }.into_any(),
                        _ => view! { <section class="not-found"><p class="eyebrow">"UNMAPPED COORDINATE"</p><h1>"Return to the ecosystem."</h1><a class="primary-button" href="#/atlas">"Open the atlas"<Arrow/></a></section> }.into_any()
                    }
                }}
            </main>
            <footer class="global-footer"><span><i class="status-dot"/>"SABER TECH / THE BIG ONE"</span><span>"TARGET ARCHITECTURE · EXECUTIVE DEMO"</span><span class="footer-right">"DESIGNED AS ONE. ACCOUNTABLE BY DOMAIN."</span></footer>
        </div>
    }
}
