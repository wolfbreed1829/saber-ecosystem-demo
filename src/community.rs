use crate::community_data::*;
use crate::data::domain;
use crate::ui::*;
use leptos::prelude::*;

#[component]
pub fn FamilyNav(active: &'static str) -> impl IntoView {
    view! { <nav class="family-nav" aria-label="Life here sections">{[("community","Neighborhood"),("learning","School campus"),("play","Play & parks"),("day","A day here"),("places","Everyday places"),("connections","What supports it")].into_iter().enumerate().map(|(i,(id,name))|view!{<a href=format!("#/{id}") class:active=active==id aria-current=if active==id{"page"}else{"false"}><small>{format!("0{}",i+1)}</small>{name}<span>"↗"</span></a>}).collect_view()}</nav> }
}

#[component]
pub fn Scene(
    asset: &'static str,
    zones: &'static [Zone],
    selected: RwSignal<usize>,
    label: &'static str,
    #[prop(optional)] journey: bool,
) -> impl IntoView {
    view! { <div class="family-scene panel"><div class="scene-top"><div><p class="eyebrow">"SABER / LIVING SYSTEMS"</p><h2>{label}</h2></div><span class="small-badge">"CONCEPT / NOT TO SCALE"</span></div><div class="scene-viewport"><div class="scene-artboard"><img src=format!("public/{asset}.svg") alt=format!("Isometric concept model of {label}")/>{zones.iter().enumerate().map(|(i,z)|{let x=(560.+(z.x-z.y)*0.83)/11.2;let y=(94.+(z.x+z.y)*0.44-35.)/7.3;view!{<button class="scene-marker" class:active=move ||selected.get()==i style=format!("left:{x}%;top:{y}%") disabled=journey aria-label=format!("Explore {}",z.name) aria-pressed=move ||(selected.get()==i).to_string() on:click=move |_|selected.set(i)><b>{format!("{:02}",i+1)}</b><span>{z.name}</span></button>}}).collect_view()}</div></div><div class="scene-foot"><span><i class="status-dot"/>{if journey{"FOLLOW THE JOURNEY STEPS BELOW"}else{"SELECT A PLACE TO EXPLORE"}}</span><span>{format!("{:02} CONNECTED SPACES",zones.len())}</span></div></div> }
}

#[component]
fn DomainLinks(ids: Vec<String>) -> impl IntoView {
    view! {<div class="place-domain-links">{ids.into_iter().map(|id|view!{<a href=format!("#/domain/{id}")><Icon name=id.clone()/><span>{domain(&id).name.clone()}</span><span>"↗"</span></a>}).collect_view()}</div>}
}

#[component]
pub fn Community() -> impl IntoView {
    let selected = RwSignal::new(2usize);
    let tab = RwSignal::new(0usize);
    view! {<section class="page family-page"><PageTitle kicker="07 / A WORLD TO GROW UP IN" title="A world to call home." description="A complete neighborhood for children, caregivers, partners, and older relatives. Explore the places that make an ordinary day possible." index="07"/><FamilyNav active="community"/>
        <div class="family-layout"><Scene asset="neighborhood" zones=&NEIGHBORHOOD selected=selected label="The family neighborhood"/>
        <aside class="family-inspector panel" aria-label="Neighborhood place details">{move ||{let z=&NEIGHBORHOOD[selected.get()];let p=place(z.id).unwrap();view!{
            <div class="family-inspector-top"><Icon name=p.icon.clone()/><span class="technical-label">{format!("PLACE {:02} / {}",selected.get()+1,p.group.to_uppercase())}</span></div><h2>{p.name.clone()}</h2><p class="family-tagline">{p.tagline.clone()}</p>
            <div class="tabs" role="tablist" aria-label="Place preview">{["Experience","Spaces","Connections"].into_iter().enumerate().map(|(i,t)|view!{<button role="tab" aria-selected=move ||(tab.get()==i).to_string() class:active=move ||tab.get()==i on:click=move |_|tab.set(i)>{t}</button>}).collect_view()}</div>
            <div class="family-inspector-copy" role="tabpanel">{move ||match tab.get(){0=>view!{<p>{p.description.clone()}</p>}.into_any(),1=>view!{<Bullets items=p.spaces.clone()/>}.into_any(),_=>view!{<p class="small-note">"Follow the service back to its accountable operating system."</p><DomainLinks ids=p.domains.clone()/>}.into_any()}}</div>
            <a class="outline-button" href=match z.id{"school"=>"#/learning".to_string(),"playground"=>"#/play".to_string(),_=>format!("#/place/{}",z.id)}>"Explore the full blueprint"<Arrow/></a>
        }}}</aside></div><div class="family-bottom"><p>"20 everyday places · 24 supporting domains · one connected community"</p><a href="#/day">"Follow a household journey →"</a></div>
    </section>}
}

#[component]
pub fn Campus(play: bool) -> impl IntoView {
    let selected = RwSignal::new(0usize);
    let tab = RwSignal::new(0usize);
    let zones: &'static [Zone] = if play { &PLAY } else { &SCHOOL };
    let active = if play { "play" } else { "learning" };
    let (title, desc, kicker, index, asset, label) = if play {
        (
            "A childhood with room to play.",
            "Adventure, quiet discovery, shared play, and space for the people who care for them.",
            "09 / PLAY, MOVEMENT & BELONGING",
            "09",
            "playground",
            "The discovery park",
        )
    } else {
        (
            "A campus for becoming.",
            "Nexus connects the learning journey. Explore the classrooms, people, meals, support, and outdoor spaces around it.",
            "08 / NEXUS LEARNING CAMPUS",
            "08",
            "school-campus",
            "The learning campus",
        )
    };
    view! {<section class="page family-page"><PageTitle kicker=kicker title=title description=desc index=index/><FamilyNav active=active/><div class="family-layout"><Scene asset=asset zones=zones selected=selected label=label/>
    <aside class="family-inspector panel" aria-label="Selected space details">{move ||{let z=&zones[selected.get()];view!{<div class="family-inspector-top"><span class="zone-number">{format!("{:02}",selected.get()+1)}</span><span class="technical-label">{z.name.to_uppercase()}</span></div><h2>{z.title}</h2><div class="tabs" role="tablist" aria-label="Space details">{["Experience","Design & operation"].into_iter().enumerate().map(|(i,t)|view!{<button role="tab" aria-selected=move ||(tab.get()==i).to_string() class:active=move ||tab.get()==i on:click=move |_|tab.set(i)>{t}</button>}).collect_view()}</div><div class="family-inspector-copy" role="tabpanel">{move ||if tab.get()==0{view!{<p>{z.body}</p>}.into_any()}else{view!{<Bullets items=z.details.iter().map(|v|v.to_string()).collect()/>}.into_any()}}</div><a class="outline-button" href=format!("#/place/{}",z.id)>"People, operation & support"<Arrow/></a>}}}</aside>
    </div><div class="family-bottom"><p>{if play{"Concept zoning. Equipment, surfacing, access, and supervision require site-specific design."}else{"Proposed educational setting. Program, ages, staffing, and site requirements remain planning inputs."}}</p><a href=if play{"https://www.cpsc.gov/Playground-Handbook"}else{"https://www.childcare.gov/consumer-education/find-and-choose-quality-child-care"} target="_blank" rel="noopener noreferrer">{if play{"Playground design reference ↗"}else{"Early-care planning reference ↗"}}</a></div></section>}
}

#[component]
pub fn Places() -> impl IntoView {
    let filter = RwSignal::new("All places");
    let search = RwSignal::new(String::new());
    let page = RwSignal::new(0usize);
    let filtered = Memo::new(move |_| {
        PLACES
            .iter()
            .filter(|p| {
                (filter.get() == "All places" || p.group == filter.get())
                    && format!("{} {} {}", p.name, p.tagline, p.description)
                        .to_lowercase()
                        .contains(&search.get().to_lowercase())
            })
            .cloned()
            .collect::<Vec<_>>()
    });
    view! {<section class="page family-page places-page"><PageTitle kicker="11 / THE EVERYDAY DIRECTORY" title="Everything around a family." description="Twenty proposed places and services, with spaces, daily routines, accountable people, and the systems that support them." index="11"/><FamilyNav active="places"/>
        <div class="directory-toolbar"><div class="filter-tabs">{["All places","Everyday living","Learning & growth","Play & belonging","Care & support"].into_iter().map(|f|view!{<button class:active=move ||filter.get()==f on:click=move |_|{filter.set(f);page.set(0);}>{f}</button>}).collect_view()}</div><label class="search-field"><span>"⌕"</span><input aria-label="Search everyday places" placeholder="Find a place…" prop:value=move ||search.get() on:input=move |ev|{search.set(event_target_value(&ev));page.set(0);}/></label></div>
        <div class="domain-grid place-grid">{move ||{let items=filtered.get();if items.is_empty(){view!{<div class="empty-state"><h2>"No matching place."</h2><p>"Try school, garden, support, or a different category."</p></div>}.into_any()}else{items.into_iter().skip(page.get()*6).take(6).map(|p|view!{<a class="domain-card place-card" href=format!("#/place/{}",p.id)><div class="card-top"><Icon name=p.icon/><span>{p.group.to_uppercase()}</span><span class="card-arrow">"↗"</span></div><h2>{p.name}</h2><p>{p.tagline}</p><div class="card-bottom"><span>"EXPLORE THIS PLACE"</span><span>{format!("{} SYSTEM CONNECTIONS",p.domains.len())}</span></div></a>}).collect_view().into_any()}}}</div>
        <div class="pagination"><span>{move ||format!("{} PLACES IN VIEW",filtered.get().len())}</span><div><button class="text-button" disabled=move ||page.get()==0 on:click=move |_|page.update(|v|*v=v.saturating_sub(1))>"← Previous"</button><span>{move ||format!("{:02} / {:02}",page.get()+1,filtered.get().len().div_ceil(6).max(1))}</span><button class="text-button" disabled={move ||(page.get()+1)*6>=filtered.get().len()} on:click=move |_|page.update(|v|*v+=1)>"Next →"</button></div></div>
    </section>}
}

#[component]
pub fn PlacePage(id: String) -> impl IntoView {
    let Some(p) = place(&id) else {
        return view!{<section class="not-found"><h1>"Place not found."</h1><a class="primary-button" href="#/places">"Explore everyday places"</a></section>}.into_any();
    };
    let tab = RwSignal::new(0usize);
    let asset = match p.id.as_str() {
        "school" | "school-kitchen" => "school-campus",
        "playground" | "parks" => "playground",
        _ => "neighborhood",
    };
    view!{<section class="page family-page place-page"><div class="domain-breadcrumb"><a href="#/places">"← Everyday places"</a><span>"/"</span><span>{p.group.clone()}</span></div><div class="domain-title"><div><p class="eyebrow">"LIFE HERE / PLACE BLUEPRINT"</p><h1>{p.name.clone()}</h1><p>{p.tagline.clone()}</p></div><Icon name=p.icon.clone()/></div><div class="place-layout">
        <aside class="place-visual panel"><p class="technical-label">"A PLACE IN THE LARGER WORLD"</p><div class="place-art"><img src=format!("public/{asset}.svg") alt="Conceptual isometric arrangement of connected community spaces"/></div><p>{p.description.clone()}</p><a class="text-link" href=match asset{"school-campus"=>"#/learning","playground"=>"#/play",_=>"#/community"}>"Locate in the neighborhood"<Arrow/></a></aside>
        <div class="place-reading panel"><div class="tabs" role="tablist" aria-label="Place blueprint chapters">{["The spaces","A normal day","People & continuity","Connected systems"].into_iter().enumerate().map(|(i,t)|view!{<button role="tab" aria-selected=move ||(tab.get()==i).to_string() class:active=move ||tab.get()==i on:click=move |_|tab.set(i)>{t}</button>}).collect_view()}</div><div class="place-chapter" role="tabpanel">{move ||match tab.get(){0=>view!{<p class="eyebrow">"THE PHYSICAL SETTING"</p><h2>"Designed around everyday use."</h2><Bullets items=p.spaces.clone()/><div class="family-note"><span>"PLANNING STATUS"</span><p>"Proposed spaces and services. Site, capacity, funding, staffing, and opening dates have not been established in this blueprint."</p></div>}.into_any(),1=>view!{<p class="eyebrow">"THE OPERATING EXPERIENCE"</p><h2>"How this place works."</h2><div class="operating-steps">{p.routine.iter().enumerate().map(|(i,v)|view!{<div><b>{format!("0{}",i+1)}</b><p>{v.clone()}</p></div>}).collect_view()}</div>}.into_any(),2=>view!{<p class="eyebrow">"ACCOUNTABLE DELIVERY"</p><h2>"People make the place dependable."</h2><p class="lead-paragraph">{p.people.clone()}</p><div class="family-note"><span>"WHEN SOMETHING CHANGES"</span><p>{p.continuity.clone()}</p></div>}.into_any(),_=>view!{<p class="eyebrow">"FOLLOW THE DEPENDENCIES"</p><h2>"Connected to real operating systems."</h2><p class="lead-paragraph">"Each connection has its own operator, inputs, maintenance needs, and evidence. Open a domain to inspect its complete blueprint."</p><DomainLinks ids=p.domains.clone()/>}.into_any()}}</div></div>
    </div></section>}.into_any()
}

#[derive(Clone, Copy)]
struct Stop {
    time: &'static str,
    title: &'static str,
    place: &'static str,
    map: usize,
    body: &'static str,
    scope: &'static str,
    authority: &'static str,
}
const JOURNEYS: [[Stop; 5]; 3] = [
    [
        Stop {
            time: "01 / SETTLE IN",
            title: "A home and a clear starting point.",
            place: "homes",
            map: 0,
            body: "A hypothetical household reviews housing terms, service options, repair contacts, and access needs. They choose the services that fit their life.",
            scope: "Housing receives tenancy and service information. Employment and clinical records remain separate.",
            authority: "The housing operator manages occupancy and repairs; the household controls optional participation.",
        },
        Stop {
            time: "02 / LEARN",
            title: "A supported handoff at Nexus.",
            place: "school",
            map: 1,
            body: "A child arrives at school with an agreed learning plan, pickup arrangements, and the support needed to participate in the day.",
            scope: "Educators see the education information required for their role. Relevant care information follows a specific authorized pathway.",
            authority: "Qualified school staff lead teaching and supervision; families participate in the learning plan.",
        },
        Stop {
            time: "03 / RECEIVE CARE",
            title: "Care stays with the care team.",
            place: "family-health",
            map: 5,
            body: "The family uses the neighborhood care hub for an appropriate appointment. A clinician determines care and any onward referral.",
            scope: "The clinic holds clinical records. A transport booking can share a destination and assistance need without exposing a diagnosis.",
            authority: "Licensed clinicians make clinical decisions, with the person's consent and applicable professional obligations.",
        },
        Stop {
            time: "04 / PLAY & CONNECT",
            title: "An afternoon that belongs to the family.",
            place: "playground",
            map: 2,
            body: "After the authorized school handoff, the household meets at the playground. A grandparent can use the shaded seating while children choose different play zones.",
            scope: "Ordinary open play does not require a student or employee performance profile.",
            authority: "Caregivers supervise ordinary visits; the parks operator maintains the space and its operating rules.",
        },
        Stop {
            time: "05 / HAVE A VOICE",
            title: "A suggestion becomes a visible proposal.",
            place: "community-center",
            map: 7,
            body: "The household suggests additional shade near the play area. The proposal receives feasibility review, community input, an authorized decision, and a published outcome.",
            scope: "The proposal includes its purpose and relevant evidence. Private household, school, and clinical records are unnecessary.",
            authority: "The community process considers the proposal; the responsible asset operator approves and delivers work within its authority.",
        },
    ],
    [
        Stop {
            time: "01 / MORNING",
            title: "Childcare that fits the care plan.",
            place: "childcare",
            map: 3,
            body: "A caregiver brings a younger child to the early learning center. Staff confirm the handoff, relevant needs, and authorized collection arrangements.",
            scope: "Childcare staff receive the information required for care and contact, with role-limited access.",
            authority: "The qualified childcare operator owns supervision and the care program.",
        },
        Stop {
            time: "02 / LEARNING",
            title: "Small discoveries throughout the day.",
            place: "childcare",
            map: 3,
            body: "Play, meals, rest, and outdoor time follow an age-appropriate routine. Staff communicate significant changes to caregivers.",
            scope: "Family updates stay within the care relationship; routine observations are not employer evaluations.",
            authority: "Qualified caregivers direct the program and raise support needs through the agreed process.",
        },
        Stop {
            time: "03 / ESSENTIALS",
            title: "A practical stop at the market.",
            place: "grocery",
            map: 7,
            body: "The household collects groceries or uses an available delivery option. Availability and substitutions are clear before collection.",
            scope: "The store needs order and delivery information, not the child's educational history.",
            authority: "The food and retail operators control quality, handling, availability, and service recovery.",
        },
        Stop {
            time: "04 / TOGETHER",
            title: "A library visit and a little play.",
            place: "library",
            map: 4,
            body: "After pickup, the family reads together at the library and chooses an activity in the nearby park.",
            scope: "Participation in optional programs has a defined registration purpose and minimal necessary records.",
            authority: "Library and parks teams manage their spaces; caregivers retain responsibility for ordinary family visits.",
        },
        Stop {
            time: "05 / AT HOME",
            title: "A home that keeps working.",
            place: "homes",
            map: 0,
            body: "A repair request reaches the housing team with the relevant access arrangements and service issue. The household can follow its progress.",
            scope: "Maintenance receives the fault and agreed access details, without broad access to family service records.",
            authority: "The housing operator assigns qualified work and communicates completion or an interim solution.",
        },
    ],
    [
        Stop {
            time: "01 / AT HOME",
            title: "Different generations, one neighborhood.",
            place: "homes",
            map: 0,
            body: "An adaptable home and accessible routes make a shared day possible for a household with younger and older relatives.",
            scope: "Access preferences are shared with the relevant service, rather than becoming a universal personal profile.",
            authority: "The household chooses its arrangements; each operator remains accountable for the service it provides.",
        },
        Stop {
            time: "02 / SUPPORT",
            title: "Help that follows the person's wishes.",
            place: "elder-care",
            map: 8,
            body: "An older relative uses agreed social or practical support. Changes in care needs are discussed with the person and appropriate professionals.",
            scope: "Support workers receive necessary care-plan information according to their role and authorization.",
            authority: "Relevant care professionals make care decisions; the resident's preferences and consent remain central.",
        },
        Stop {
            time: "03 / MOVE",
            title: "A route to the places that matter.",
            place: "transit",
            map: 5,
            body: "An accessible transport option connects a care visit, the market, or community activity. Assistance requirements are made clear to the transport team.",
            scope: "Transport receives journey and assistance details; clinical records remain with clinicians.",
            authority: "The transport operator owns vehicle readiness, qualified staffing, and the agreed journey.",
        },
        Stop {
            time: "04 / BELONG",
            title: "Shared space across generations.",
            place: "parks",
            map: 2,
            body: "Family members meet near the play park, where accessible paths, seating, quiet areas, and activity choices let everyone take part differently.",
            scope: "Using public space does not require a combined family health or work record.",
            authority: "The parks operator maintains the setting; each person chooses their level of participation.",
        },
        Stop {
            time: "05 / CONTRIBUTE",
            title: "Experience becomes part of community life.",
            place: "community-center",
            map: 7,
            body: "A resident joins a community discussion or volunteers for an appropriate activity. Participation can happen in person or through an accessible alternative.",
            scope: "Only the information needed for the proposal or activity enters that process.",
            authority: "The agreed governance process defines who advises, who decides, and how decisions can be reviewed.",
        },
    ],
];

#[component]
pub fn Day() -> impl IntoView {
    let household = RwSignal::new(0usize);
    let step = RwSignal::new(0usize);
    let selected = RwSignal::new(0usize);
    Effect::new(move |_| {
        selected.set(JOURNEYS[household.get()][step.get()].map);
    });
    view! {<section class="page family-page journey-page"><PageTitle kicker="10 / A HOUSEHOLD JOURNEY" title="See the world through a family's day." description="Three illustrative journeys connect places, services, permissions, and the people who make decisions." index="10"/><FamilyNav active="day"/>
        <div class="journey-controls"><div class="section-switcher">{["School-age family","Young family","Across generations"].into_iter().enumerate().map(|(i,t)|view!{<button class:active=move ||household.get()==i on:click=move |_|{household.set(i);step.set(0);}>{t}</button>}).collect_view()}</div><span class="technical-label">"ILLUSTRATIVE EXPERIENCE / NOT AN AVAILABLE SERVICE"</span></div>
        <div class="journey-layout"><div class="journey-map"><Scene asset="neighborhood" zones=&NEIGHBORHOOD selected=selected label="Follow the household" journey=true/><div class="journey-track" aria-label="Journey steps">{(0..5).map(|i|view!{<button class:active=move ||step.get()==i aria-label=format!("Journey step {}",i+1) aria-pressed=move ||(step.get()==i).to_string() on:click=move |_|step.set(i)><span>{format!("0{}",i+1)}</span><b>{move ||JOURNEYS[household.get()][i].time.split(" / ").nth(1).unwrap_or("")}</b></button>}).collect_view()}</div></div>
        <article class="journey-detail panel">{move ||{let s=JOURNEYS[household.get()][step.get()];view!{<p class="eyebrow">{s.time}</p><h2>{s.title}</h2><p class="journey-body">{s.body}</p><div class="scope-card"><span>"RECORDS & PERMISSIONS"</span><p>{s.scope}</p></div><div class="scope-card"><span>"WHO DECIDES"</span><p>{s.authority}</p></div><div class="journey-actions"><a href=format!("#/place/{}",s.place)>"Explore this place ↗"</a><button class="outline-button" on:click=move |_|step.update(|i|*i=(*i+1)%5)>{move ||if step.get()==4{"Restart journey"}else{"Next moment"}}<Arrow/></button></div>}}}</article></div>
    </section>}
}

const SUPPORT: [(&str, &str, &str, [(&str, &str); 4], &str); 5] = [
    (
        "A dependable home",
        "housing",
        "Comfort starts upstream.",
        [
            ("water", "Drinking water & sanitation"),
            ("energy", "Power, heat & cooling"),
            ("construction", "Building care & qualified repairs"),
            ("continuity", "Interruption & recovery planning"),
        ],
        "Resident service requests reach the accountable housing operator. Aether coordinates work; it does not grant unrelated services access to household records.",
    ),
    (
        "A school day",
        "education",
        "Learning depends on more than a classroom.",
        [
            ("food", "Reliable meals & qualified ingredients"),
            ("community", "Family support & recreation"),
            ("logistics", "Arrival, pickup & accessible transport"),
            ("digital", "Scoped learning records & coordination"),
        ],
        "Nexus supports the learning journey. Educators lead instruction, food teams own meal service, and families participate through defined permissions.",
    ),
    (
        "A visit to the park",
        "community",
        "Public space is an operating commitment.",
        [
            ("construction", "Paths, structures & qualified repairs"),
            ("water", "Drinking water, cleaning & planting"),
            ("land", "Suitable ground & landscape care"),
            ("continuity", "Inspection, closures & response"),
        ],
        "The parks operator owns maintenance and operating rules. The community process provides a route to propose improvements and follow decisions.",
    ),
    (
        "A family meal",
        "grocery",
        "A stocked shelf is the end of a chain.",
        [
            ("agriculture", "Food production & renewed growing inputs"),
            ("food", "Processing, preservation & quality"),
            ("logistics", "Storage, cold transport & distribution"),
            ("recovery", "Qualified recovery of waste & packaging"),
        ],
        "Household demand informs supply planning. Food quality remains with qualified operators, and missing internal inputs stay visible.",
    ),
    (
        "A care appointment",
        "health",
        "Care requires capability and continuity.",
        [
            ("medical", "Qualified medicines, diagnostics & supplies"),
            ("education", "Qualified people & professional development"),
            ("logistics", "Accessible transport & referral connections"),
            ("continuity", "Staffing, backup & advanced-care pathways"),
        ],
        "Clinical records and decisions stay with the care team. A booking or transport workflow receives only the information needed for its role.",
    ),
];
#[component]
pub fn Connections() -> impl IntoView {
    let selected = RwSignal::new(0usize);
    let boundary_tab = RwSignal::new(0usize);
    view! {<section class="page family-page connections-page"><PageTitle kicker="12 / THE SYSTEM BEHIND THE EXPERIENCE" title="Every ordinary moment has a whole world behind it." description="Choose an experience to reveal its physical dependencies and the boundary around coordination and decision-making." index="12"/><FamilyNav active="connections"/>
        <div class="section-switcher">{SUPPORT.iter().enumerate().map(|(i,(name,_,_,_,_))|view!{<button class:active=move ||selected.get()==i on:click=move |_|selected.set(i)>{*name}</button>}).collect_view()}</div>
        {move ||{let(_,id,title,nodes,boundary)=SUPPORT[selected.get()];view!{<div class="support-layout"><div class="support-diagram panel"><p class="technical-label">"PHYSICAL CAPABILITY / FOLLOW EACH CONNECTION"</p><div class="support-center"><Icon name=id.to_string()/><h2>{title}</h2><a href=format!("#/domain/{id}")>{domain(id).name.clone()}" ↗"</a></div><div class="support-wires" aria-hidden="true"><i/><i/><i/><i/></div><div class="support-nodes">{nodes.into_iter().map(|(id,label)|view!{<a href=format!("#/domain/{id}")><Icon name=id.to_string()/><span>{label}</span><small>"OPEN RESPONSIBLE DOMAIN ↗"</small></a>}).collect_view()}</div></div><aside class="support-boundary panel"><p class="eyebrow">"INTEGRATED. SCOPED. ACCOUNTABLE."</p><h2>"Connection has boundaries."</h2><div class="tabs" role="tablist" aria-label="Connection boundaries">{["Roles","Digital","Authority"].into_iter().enumerate().map(|(i,t)|view!{<button role="tab" aria-selected=move ||(boundary_tab.get()==i).to_string() class:active=move ||boundary_tab.get()==i on:click=move |_|boundary_tab.set(i)>{t}</button>}).collect_view()}</div><div class="boundary-copy" role="tabpanel"><p>{move ||match boundary_tab.get(){0=>boundary,1=>"Aether workspaces coordinate requests. ATMP CORE carries authorized work and evidence. Access is limited to the task and role. The household can understand what is shared, why it is needed, and which service is responsible.",_=>"Domain operators and qualified professionals retain their decisions. Axiom checks work against the applicable policy and evidence boundary. Resident participation has a defined scope and review route; it does not override clinical or other professional authority."}}</p></div><a class="text-link" href="#/governance">"Explore authority & rights"<Arrow/></a></aside></div>}}}
    </section>}
}
