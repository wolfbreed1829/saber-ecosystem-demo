use leptos::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local};
#[component]
pub fn Arrow() -> impl IntoView {
    view! { <svg class="arrow" viewBox="0 0 24 24" fill="none" aria-hidden="true"><path d="M4 12h15M13 5l7 7-7 7" stroke="currentColor" stroke-width="1.5"/></svg> }
}
#[component]
pub fn Logo() -> impl IntoView {
    view! { <a class="brand" href="#/" aria-label="Saber Tech home"><svg viewBox="0 0 40 40" fill="none" aria-hidden="true"><path d="M7 8h28L20 20h14L11 34l8-13H5l12-8H7Z" fill="currentColor"/></svg><span>"SABER"<b>"TECH"</b><small>"INTEGRATED WORLD SYSTEMS"</small></span></a> }
}
#[component]
pub fn Header(route: RwSignal<String>, reduced: RwSignal<bool>) -> impl IntoView {
    let copied = RwSignal::new(false);
    let copy_error = RwSignal::new(false);
    let nav = [
        ("atlas", "Atlas"),
        ("community", "Life here"),
        ("systems", "Domains"),
        ("flows", "Flows"),
        ("architecture", "Architecture"),
        ("governance", "Governance"),
        ("roadmap", "Roadmap"),
    ];
    view! {
        <header class="global-header"><Logo/>
            <nav aria-label="Main navigation" class:entrance-nav=move || route.get().is_empty()>{nav.into_iter().enumerate().map(|(i,(path,label))|view! {
                <a href=format!("#/{path}") class:active=move || route.get()==path || (path=="systems" && route.get().starts_with("domain/")) || (path=="community" && (matches!(route.get().as_str(),"learning"|"play"|"day"|"places"|"connections")||route.get().starts_with("place/"))) aria-current=move || if route.get()==path {"page"} else {"false"}><small>{format!("{:02}",i+1)}</small>{label}</a>
            }).collect_view()}</nav>
            <div class="header-actions"><button class="icon-button motion-button" title="Toggle animation" aria-label="Toggle animation" aria-pressed=move || (!reduced.get()).to_string() on:click=move |_|reduced.update(|v|*v=!*v)><span>{move || if reduced.get(){"Ⅱ"}else{"≈"}}</span></button>
                <button class="share-button" on:click=move |_| {spawn_local(async move {let url=window().location().href().unwrap_or_default();if JsFuture::from(window().navigator().clipboard().write_text(&url)).await.is_ok() {copied.set(true);copy_error.set(false);} else {copy_error.set(true);}});} ><span>{move || if copied.get(){"Link copied"}else if copy_error.get(){"Copy address bar"}else{"Copy link"}}</span><svg viewBox="0 0 20 20" fill="none" aria-hidden="true"><path d="M8 5H4v12h12v-4M10 3h7v7M17 3 8 12" stroke="currentColor" stroke-width="1.3"/></svg></button>
                <span class="sr-only" role="status">{move || if copied.get(){"Page link copied to clipboard"}else{""}}</span>
            </div>
        </header>
    }
}
#[component]
pub fn PageTitle(
    kicker: &'static str,
    title: &'static str,
    description: &'static str,
    index: &'static str,
) -> impl IntoView {
    view! {<div class="page-title"><div><p class="eyebrow"><span/> {kicker}</p><h1>{title}</h1><p class="page-description">{description}</p></div><span class="page-index">{index}<small>" / 12"</small></span></div>}
}
pub fn icon_svg(name: &str) -> String {
    let body = match name {
        "continuity" => {
            "<path d='m20 3 14 5v12c0 9-14 17-14 17S6 29 6 20V8Z'/><path d='m12 20 6 6 11-13'/>"
        }
        "play" => {
            "<path d='M5 35 13 6h14l8 29M10 16h20M16 16v13h8V16M13 35h14'/><path d='m3 35 34 0'/>"
        }
        "water" => {
            "<path d='M20 4S8 16 8 23a12 12 0 0 0 24 0C32 16 20 4 20 4Z'/><path d='M13 24a7 7 0 0 0 7 7'/>"
        }
        "energy" | "fuel" => "<path d='m23 3-15 20h11l-2 14 15-21H21Z'/>",
        "health" | "medical" | "veterinary" => "<path d='M15 5h10v10h10v10H25v10H15V25H5V15h10Z'/>",
        "housing" | "community" => "<path d='M4 19 20 5l16 14M9 16v20h22V16M17 36V24h7v12'/>",
        "agriculture" | "land" | "food" => {
            "<path d='M20 36V17M20 25C5 24 5 12 5 12s15-2 15 13ZM20 18C20 4 35 4 35 4s1 14-15 14Z'/>"
        }
        "logistics" => {
            "<path d='M3 10h22v20H3ZM25 17h8l5 7v6H25M7 30a4 4 0 1 0 8 0M27 30a4 4 0 1 0 8 0'/>"
        }
        "education" => "<path d='m3 15 17-9 17 9-17 9ZM10 20v10c8 6 12 6 20 0V20M37 15v14'/>",
        "governance" | "finance" => {
            "<path d='m4 13 16-9 16 9ZM4 35h32M8 17v14M16 17v14M24 17v14M32 17v14'/>"
        }
        "recovery" => {
            "<path d='M30 10a14 14 0 0 0-23 4M30 3v9H21M10 30a14 14 0 0 0 23-4M10 37v-9h9'/>"
        }
        "digital" | "electronics" => {
            "<rect x='9' y='9' width='22' height='22' rx='2'/><rect x='15' y='15' width='10' height='10'/><path d='M15 3v6M25 3v6M15 31v6M25 31v6M3 15h6M3 25h6M31 15h6M31 25h6'/>"
        }
        "grocery" | "household" => {
            "<path d='M5 14h30l-3 21H8ZM12 16V9a8 8 0 0 1 16 0v7M15 23v6M25 23v6'/>"
        }
        _ => {
            "<path d='M5 35V18l10-7v9l10-7v8h10v14ZM5 35h30M28 21V5h6v16M11 26v4M19 26v4M27 26v4'/>"
        }
    };
    format!(
        "<svg viewBox='0 0 40 40' fill='none' stroke='currentColor' stroke-width='1.4' stroke-linecap='round' stroke-linejoin='round' aria-hidden='true'>{body}</svg>"
    )
}
#[component]
pub fn Icon(name: String) -> impl IntoView {
    view! {<span class="domain-icon" inner_html=icon_svg(&name)></span>}
}
#[component]
pub fn Bullets(items: Vec<String>) -> impl IntoView {
    view! {<ul class="detail-list">{items.into_iter().map(|item|view!{<li><span>"↗"</span><p>{item}</p></li>}).collect_view()}</ul>}
}
