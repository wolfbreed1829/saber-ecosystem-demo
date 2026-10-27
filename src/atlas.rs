use crate::data::{domain, number};
use crate::ui::*;
use leptos::prelude::*;

const DISTRICTS: [(&str, &str, f64, f64); 11] = [
    ("agriculture", "Food production", 50., 18.),
    ("energy", "Energy", 66., 25.),
    ("water", "Water", 80., 42.),
    ("grocery", "Grocery & essentials", 35., 32.),
    ("digital", "Aether core", 51., 25.),
    ("health", "Healthcare", 66., 51.),
    ("housing", "Housing", 20., 39.),
    ("machinery", "Manufacturing", 35., 59.),
    ("fuel", "Fuel & logistics", 51., 74.),
    ("education", "Nexus campus", 16., 15.),
    ("governance", "Civic commons", 82., 76.),
];
fn coordinate(id: &str) -> (f64, f64) {
    DISTRICTS
        .iter()
        .find(|(d, _, _, _)| *d == id)
        .map(|(_, _, x, y)| (*x, *y))
        .unwrap_or(match id {
            "construction" => (29., 62.),
            "continuity" => (67., 80.),
            "community" => (82., 76.),
            "food" => (35., 32.),
            "medical" => (66., 51.),
            "land" => (50., 18.),
            "logistics" => (51., 74.),
            "materials" => (35., 59.),
            _ => (42., 83.),
        })
}
fn supply(id: &str) -> &'static str {
    match id {
        "water" => "Water & sanitation",
        "energy" => "Power, heat & cooling",
        "construction" => "Construction & repairs",
        "continuity" => "Maintenance continuity",
        "education" => "Qualified people & learning",
        "community" => "Household support",
        "food" => "Prepared & preserved food",
        "agriculture" => "Food & growing inputs",
        "medical" => "Medicines & equipment",
        "land" => "Land & resource access",
        "logistics" => "Transport & distribution",
        "materials" => "Qualified materials",
        "machinery" => "Tools, parts & equipment",
        "recovery" => "Material recovery",
        "governance" => "Rules & accountability",
        "digital" => "Authorized coordination",
        "finance" => "Funding & obligations",
        "housing" => "Homes & facilities",
        "electronics" => "Sensors & controls",
        "fuel" => "Fuel & charging",
        _ => "Qualified supply & service",
    }
}
fn links(id: &str, mode: usize) -> Vec<(String, String, String)> {
    if mode == 1 {
        return vec![
            (
                "digital".into(),
                "Aether · scoped workspaces".into(),
                "#/architecture".into(),
            ),
            (
                "education".into(),
                "Nexus · learning permissions".into(),
                "#/domain/education".into(),
            ),
            (
                "governance".into(),
                "ATMP CORE · authorized work".into(),
                "#/architecture".into(),
            ),
        ];
    }
    if mode == 2 {
        return vec![
            (
                "governance".into(),
                "Policy, review & resident voice".into(),
                "#/governance".into(),
            ),
            (
                "health".into(),
                "Clinical authority stays scoped".into(),
                "#/domain/health".into(),
            ),
            (
                "digital".into(),
                "Axiom · policy & evidence checks".into(),
                "#/architecture".into(),
            ),
        ];
    }
    let ids: Vec<String> = match id {
        "housing" => vec!["water", "energy", "construction", "continuity"]
            .into_iter()
            .map(String::from)
            .collect(),
        "education" => vec!["food", "community", "logistics", "water"]
            .into_iter()
            .map(String::from)
            .collect(),
        "governance" => vec!["community", "education", "continuity", "digital"]
            .into_iter()
            .map(String::from)
            .collect(),
        _ => domain(id)
            .links
            .iter()
            .filter(|v| v.as_str() != id)
            .take(4)
            .cloned()
            .collect(),
    };
    ids.into_iter()
        .map(|v| {
            let label = supply(&v).to_string();
            let href = format!("#/domain/{v}");
            (v, label, href)
        })
        .collect()
}

struct RoutedLink {
    label: String,
    href: String,
    index: usize,
    x: f64,
    y: f64,
    end_x: f64,
    end_y: f64,
}
fn routed_links(id: &str, mode: usize) -> Vec<RoutedLink> {
    let (x, y) = coordinate(id);
    let mut items = links(id, mode)
        .into_iter()
        .enumerate()
        .map(|(index, (target, label, href))| {
            let (ex, ey) = coordinate(&target);
            RoutedLink {
                label,
                href,
                index,
                x: (x * 0.4 + ex * 0.6).clamp(24., 76.),
                y: y * 0.4 + ey * 0.6,
                end_x: ex,
                end_y: ey,
            }
        })
        .collect::<Vec<_>>();
    items.sort_by(|a, b| a.y.total_cmp(&b.y));
    let mut floor = 17.;
    for item in &mut items {
        item.y = item.y.max(floor);
        floor = item.y + 13.;
    }
    let mut ceiling = 85.;
    for item in items.iter_mut().rev() {
        item.y = item.y.min(ceiling);
        ceiling = item.y - 13.;
    }
    items.sort_by_key(|item| item.index);
    items
}
#[component]
fn ConnectionOverlay(selected: RwSignal<&'static str>, mode: RwSignal<usize>) -> impl IntoView {
    let (x, y) = coordinate(selected.get());
    let items = routed_links(selected.get(), mode.get());
    view! {<svg class="atlas-connection-lines" viewBox="0 0 1120 730" fill="none" aria-hidden="true">{items.iter().map(|item|{
        let cx=(2.*item.x-(x+item.end_x)/2.)*11.2;let cy=(2.*item.y-(y+item.end_y)/2.)*7.3;
        let path=format!("M {} {} Q {cx} {cy} {} {}",x*11.2,y*7.3,item.end_x*11.2,item.end_y*7.3);
        view!{<path class="connection-shadow" d=path.clone()/><path class="connection-live" d=path/><circle cx=(item.end_x*11.2).to_string() cy=(item.end_y*7.3).to_string() r="6" fill="#b4edff"/>}
    }).collect_view()}</svg>{items.into_iter().map(|item|view!{<a class="supply-label" style=format!("left:{}%;top:{}%",item.x,item.y) href=item.href title=format!("Follow {}",item.label)><small>{format!("0{}",item.index+1)}</small>{item.label.clone()}<span>"↗"</span></a>}).collect_view()}}
}
#[component]
pub fn Atlas() -> impl IntoView {
    let selected = RwSignal::new("housing");
    let tab = RwSignal::new(0usize);
    let mode = RwSignal::new(0usize);
    let revealed = RwSignal::new(false);
    view! {<section class="page atlas-page atlas-expanded"><PageTitle kicker="01 / THE PHYSICAL ECOSYSTEM" title="A world that works together." description="Select a district to reveal its connections. Follow a supply line, inspect coordination, or see who holds decision authority." index="01"/>
        <div class="atlas-layout"><div class="map-panel panel"><div class="map-toolbar"><span class="technical-label">"SYSTEM ATLAS / 11 DISTRICT ENTRIES"</span><span class="small-badge">"CONCEPT MODEL"</span></div><div class="atlas-mode-bar" role="group" aria-label="Connection layers">{["Physical resources","Digital coordination","Decision authority"].into_iter().enumerate().map(|(i,t)|view!{<button class:active=move ||mode.get()==i aria-pressed=move ||(mode.get()==i).to_string() on:click=move |_|{mode.set(i);revealed.set(true);tab.set(1);}>{t}</button>}).collect_view()}<button class="hide-connections" on:click=move |_|revealed.update(|v|*v=!*v)>{move ||if revealed.get(){"Hide lines"}else{"Reveal lines"}}</button></div>
        <div class="atlas-canvas"><div class="atlas-board" class:has-connections=move ||revealed.get()><img src="public/atlas-extended.svg" alt="Eleven conceptual district entries including Nexus learning campus and civic commons alongside homes, healthcare, food, energy, water, industry, and transport"/>
            {move ||if revealed.get(){view!{<ConnectionOverlay selected=selected mode=mode/>}.into_any()}else{().into_any()}}
            {DISTRICTS.into_iter().map(|(id,label,x,y)|view!{<button class="map-marker" class:chosen=move ||selected.get()==id class:quiet=move ||revealed.get()&&selected.get()!=id style=format!("left:{x}%;top:{y}%") aria-label=format!("Select {label}") aria-pressed=move ||(selected.get()==id).to_string() on:click=move |_|{selected.set(id);revealed.set(true);tab.set(0);} ><i/><span>{label}<small>{format!("DOMAIN {}",number(id))}</small></span></button>}).collect_view()}
        </div></div><div class="map-bottom"><span><i class="status-dot"/>{move ||if revealed.get(){"FOLLOW A LABELED CONNECTION ↗"}else{"SELECT A DISTRICT TO REVEAL CONNECTIONS"}}</span><a href="#/community">"Explore family neighborhood ↗"</a></div></div>
        <aside class="inspector panel" aria-label="Selected district details">{move ||{let d=domain(selected.get());view!{<div class="inspector-heading"><Icon name=d.id.clone()/><span class="technical-label">{format!("DOMAIN {} / {}",number(&d.id),d.sector.to_uppercase())}</span></div><h2>{d.name.clone()}</h2><p class="inspector-tagline">{d.tagline.clone()}</p><div class="tabs" role="tablist" aria-label="District detail">{["Purpose","Dependencies","Evidence required"].into_iter().enumerate().map(|(i,t)|view!{<button role="tab" aria-selected=move ||(tab.get()==i).to_string() class:active=move ||tab.get()==i on:click=move |_|{tab.set(i);if i==1{revealed.set(true);}}>{t}</button>}).collect_view()}</div>
        <div class="inspector-content" role="tabpanel">{move ||match tab.get(){0=>view!{<p>{d.summary.clone()}</p><div class="atlas-related"><a href=match d.id.as_str(){"education"=>"#/learning","governance"=>"#/place/community-center",_=>"#/community"}>{match d.id.as_str(){"education"=>"Explore the Nexus campus →","governance"=>"Visit the civic commons →",_=>"See this through family life →"}}</a></div>}.into_any(),1=>view!{<p class="atlas-layer-caption">{match mode.get(){0=>"Physical inputs and service responsibilities.",1=>"Shared coordination with permissions scoped to the task.",_=>"Different decisions belong to different authorities."}}</p><div class="atlas-link-list">{links(selected.get(),mode.get()).into_iter().enumerate().map(|(i,(_,label,href))|view!{<a href=href><small>{format!("0{}",i+1)}</small><span>{label}</span><b>"↗"</b></a>}).collect_view()}</div>}.into_any(),_=>view!{<p>{d.proof.clone()}</p><p class="evidence-status">"STATUS / NO DOMAIN OPERATING EVIDENCE PUBLISHED"</p>}.into_any()}}</div><a class="outline-button" href=format!("#/domain/{}",d.id)>"Explore this domain"<Arrow/></a>}}}</aside></div>
    </section>}
}
