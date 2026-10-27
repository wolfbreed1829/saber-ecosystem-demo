use crate::data::{DOMAINS, domain, number};
use crate::ui::*;
use leptos::prelude::*;

struct Flow {
    name: &'static str,
    title: &'static str,
    description: &'static str,
    stages: [(&'static str, &'static str, &'static str); 5],
    recovery: &'static str,
}
const FLOWS: [Flow; 5] = [
    Flow {
        name: "Food & groceries",
        title: "From a seed to a stocked shelf.",
        description: "Follow the physical chain behind a Saber grocery store. Every link has its own inputs, operator, quality standard, and replacement needs.",
        stages: [
            (
                "land",
                "Resource base",
                "Suitable land, water, seed, and nutrient inputs establish productive capacity.",
            ),
            (
                "agriculture",
                "Grow & renew",
                "Produce the agreed food basket and renew the biological inputs needed for the next season.",
            ),
            (
                "food",
                "Process & preserve",
                "Turn harvests into accepted food with sanitation, packaging, storage, and traceability.",
            ),
            (
                "logistics",
                "Store & distribute",
                "Move qualified lots through warehouses and cold transport with measurable losses.",
            ),
            (
                "grocery",
                "Serve households",
                "Provide the essentials basket, transparent prices, accessible stores, and delivery.",
            ),
        ],
        recovery: "Recover qualified organic material and reusable packaging. Record nutrients, spoilage, and material losses that still need replenishment.",
    },
    Flow {
        name: "Energy & mobility",
        title: "From energy source to movement.",
        description: "Trace liquid fuel and electrical pathways beyond the station. The source, conversion assets, storage, vehicle, and replacement components all matter.",
        stages: [
            (
                "land",
                "Resource source",
                "Identify suitable energy resources or feedstocks within the declared boundary.",
            ),
            (
                "energy",
                "Generate & convert",
                "Produce useful electricity, heat, or a qualified fuel pathway and measure conversion losses.",
            ),
            (
                "materials",
                "Produce & maintain",
                "Supply materials, processing inputs, storage equipment, and required replacement components.",
            ),
            (
                "fuel",
                "Store & dispense",
                "Operate stations, charging, metering, inventory, and appropriate safety controls.",
            ),
            (
                "logistics",
                "Move people & goods",
                "Deliver useful transport through qualified vehicles, operators, and maintenance.",
            ),
        ],
        recovery: "Recover appropriate batteries, metals, and equipment. Include the resources and energy required to manufacture their replacements.",
    },
    Flow {
        name: "Housing & places",
        title: "From raw material to a lasting home.",
        description: "A housing system includes the ability to build, service, renovate, and replace. Follow the assets through their complete useful life.",
        stages: [
            (
                "land",
                "Qualify the site",
                "Establish suitable land, access, environmental conditions, and infrastructure requirements.",
            ),
            (
                "materials",
                "Produce materials",
                "Supply accepted timber, binders, metals, glass, insulation, and other required inputs.",
            ),
            (
                "construction",
                "Build & commission",
                "Combine qualified trades, tools, designs, inspections, and complete utility connections.",
            ),
            (
                "housing",
                "Live & maintain",
                "Provide accessible homes, clear tenancy terms, responsive repairs, and community facilities.",
            ),
            (
                "recovery",
                "Renew & recover",
                "Renovate assets, recover useful materials, and account for losses and replacement work.",
            ),
        ],
        recovery: "Feed accepted recovered materials into future production. Fund the next roof, fixture, appliance, and utility repair before they become failures.",
    },
    Flow {
        name: "Healthcare",
        title: "From clinical need to continuity of care.",
        description: "Healthcare independence requires people, supplies, diagnostic capability, and advanced-care pathways. Clinical authority remains with qualified professionals.",
        stages: [
            (
                "education",
                "Develop expertise",
                "Provide learning and practice pathways with appropriate professional qualification.",
            ),
            (
                "medical",
                "Qualify supplies",
                "Establish accepted medicine, diagnostic, sterile-supply, and medical-equipment pathways.",
            ),
            (
                "health",
                "Deliver care",
                "Support the clinical encounter, professional decisions, private records, and treatment.",
            ),
            (
                "community",
                "Support recovery",
                "Connect follow-up, rehabilitation, accessible transport, and household support.",
            ),
            (
                "continuity",
                "Maintain continuity",
                "Prepare for staffing gaps, equipment failures, emergency needs, and service interruptions.",
            ),
        ],
        recovery: "Appropriate sterilization, safe waste handling, and maintained equipment support continuity. Necessary outside care remains explicit while internal gaps are open.",
    },
    Flow {
        name: "Industrial renewal",
        title: "Build the machines that build the system.",
        description: "Long-term independence reaches beyond consumables. Follow the equipment, precision tools, materials, and knowledge required to replace productive capacity.",
        stages: [
            (
                "materials",
                "Qualify materials",
                "Supply materials with the properties required by the finished component or machine.",
            ),
            (
                "machinery",
                "Make tools & parts",
                "Establish fabrication, tooling, metrology, and qualified production processes.",
            ),
            (
                "electronics",
                "Integrate controls",
                "Supply and test sensors, power electronics, computing, and control components.",
            ),
            (
                "construction",
                "Install & maintain",
                "Commission productive assets and support preventive maintenance and repair.",
            ),
            (
                "recovery",
                "Recover & reproduce",
                "Recover suitable material and demonstrate a pathway to produce the next generation of assets.",
            ),
        ],
        recovery: "Trace the upstream tools and equipment recursively. A machine shop still has open dependencies if it cannot obtain or reproduce its critical tooling.",
    },
];

#[component]
pub fn Flows() -> impl IntoView {
    let active = RwSignal::new(0usize);
    let step = RwSignal::new(0usize);
    view! {<section class="page flows-page"><PageTitle kicker="03 / THE PRODUCTION FLOWS" title="Follow what makes it possible." description="Five essential journeys reveal how resources, production, services, and recovery connect." index="03"/>
        <div class="section-switcher">{FLOWS.iter().enumerate().map(|(i,f)|view!{<button class:active=move ||active.get()==i on:click=move |_|{active.set(i);step.set(0);}>{f.name}</button>}).collect_view()}</div>
        {move ||{let f=&FLOWS[active.get()];view!{<div class="flow-board panel"><div class="flow-intro"><p class="eyebrow">{format!("FLOW 0{} / PHYSICAL DEPENDENCIES",active.get()+1)}</p><h2>{f.title}</h2><p>{f.description}</p></div><div class="flow-chain">{f.stages.iter().enumerate().map(|(i,(id,label,_))|view!{<button class="flow-node" class:active=move ||step.get()==i on:click=move |_|step.set(i)><span class="node-index">{format!("0{}",i+1)}</span><Icon name=(*id).to_string()/><b>{*label}</b><span class="node-arrow">"→"</span></button>}).collect_view()}</div><div class="recovery-line"><span>"↶"</span><p>{f.recovery}</p><span>"RECOVERY LOOP"</span></div></div>
        <div class="flow-explanation panel">{move ||{let(id,label,text)=f.stages[step.get()];view!{<div class="explanation-number">{format!("0{}",step.get()+1)}</div><div><p class="eyebrow">"SELECTED STAGE"</p><h3>{label}</h3><p>{text}</p></div><a class="outline-button" href=format!("#/domain/{id}")>"Explore domain"<Arrow/></a>}}}</div>}}}
    </section>}
}

const LAYERS: [(&str, &str, &str, &str); 6] = [
    (
        "01",
        "PEOPLE & SERVICES",
        "The experience people rely on",
        "Homes, care, learning, groceries, transport, and community services have clear promises, accessible terms, and accountable operators. People can use each service independently.",
    ),
    (
        "02",
        "AETHER WORKSPACES",
        "A coordinated point of access",
        "Aether resolves the person, organization, workspace, and permitted role. It brings applications and tasks together while preserving the boundaries between operating businesses.",
    ),
    (
        "03",
        "ATMP-CORE",
        "Shared mechanisms, precise contracts",
        "Reusable authority checks, workflows, transactions, events, audit, and ledger integration support the domains. A common mechanism does not take ownership of a clinical or academic decision.",
    ),
    (
        "04",
        "ACCOUNTABLE DOMAINS",
        "The business owns its decisions",
        "Each operator owns authoritative records, professional responsibilities, delivery, support, and financial obligations. Explicit integration contracts exchange only the facts required for a workflow.",
    ),
    (
        "05",
        "PHYSICAL OPERATIONS",
        "The infrastructure beneath the interface",
        "Facilities, resource stocks, productive assets, maintenance, qualified people, and material flows perform the real work. Industrial controls remain separate from public applications and optional AI.",
    ),
    (
        "06",
        "EVIDENCE & OVERSIGHT",
        "Claims that can be inspected",
        "Permitted reporting connects source records, service outcomes, costs, dependencies, and recovery evidence. Corporate oversight and Axiom community accountability receive only the information their authority allows.",
    ),
];
#[component]
pub fn Architecture() -> impl IntoView {
    let active = RwSignal::new(2usize);
    view! {<section class="page architecture-page"><PageTitle kicker="04 / THE CONNECTED ARCHITECTURE" title="One ecosystem. Clear boundaries." description="Shared technology coordinates the work. Each operating domain retains its authority and responsibility." index="04"/>
        <div class="architecture-layout"><div class="architecture-stack panel"><p class="technical-label">"LOGICAL SYSTEM TOPOLOGY"</p>{LAYERS.iter().enumerate().map(|(i,(n,name,_,_))|view!{<button class="architecture-layer" class:active=move ||active.get()==i on:click=move |_|active.set(i)><span>{*n}</span><b>{*name}</b><i>"↗"</i></button>}).collect_view()}<div class="stack-caption">"SELECT A LAYER TO INSPECT ITS RESPONSIBILITY"</div></div>
        <div class="architecture-detail panel">{move ||{let(n,name,title,body)=LAYERS[active.get()];view!{<div class="large-outline-number">{n}</div><p class="eyebrow">{name}</p><h2>{title}</h2><p class="lead-paragraph">{body}</p><div class="boundary-grid"><div><span>"IDENTITY"</span><p>"Access is scoped to the person, role, workspace, and owning domain."</p></div><div><span>"AUTHORITY"</span><p>"Corporate, professional, and community powers stay distinct."</p></div><div><span>"CONTINUITY"</span><p>"Essential work has a defined response to platform or network failure."</p></div></div><a class="text-link" href="#/domain/digital">"Read the digital operating blueprint"<Arrow/></a>}}}</div></div>
    </section>}
}

const AUTHORITIES: [(&str, &str, &str, &str, &str); 4] = [
    (
        "Corporate",
        "Saber Tech & operators",
        "Own the obligation.",
        "Entity-specific bodies appoint operators, authorize capital, approve contracts, and oversee the obligations their actual rights cover. The parent also has to deliver the shared services it promises.",
        "An ownership interest does not create unlimited access to records, authority over professional decisions, or a right to use another entity’s funds.",
    ),
    (
        "Community",
        "Axiom & members",
        "A meaningful voice, with defined power.",
        "An adopted charter can delegate community budgets, service priorities, representatives, and reviews. Proposals move through authority checks, costing, voting, authorized execution, and a visible outcome.",
        "A vote stays within its entrusted resources. It cannot change a diagnosis, a grade, an individual lending decision, or the restrictions attached to treatment funds.",
    ),
    (
        "Professional",
        "Qualified domain leaders",
        "Expert judgment stays accountable.",
        "Clinicians, educators, engineers, and other qualified leaders own consequential decisions within their professional responsibilities. Their work has appropriate acceptance, review, records, and independent escalation.",
        "Operational targets, company ownership, or a community vote cannot substitute for required professional competence and responsibility.",
    ),
    (
        "Individual",
        "Employees, partners & households",
        "Participation preserves choice.",
        "Members choose services, understand terms, access legitimate personal records, raise complaints, and participate in community decisions. Departure, hardship, disability, and changing circumstances have practical procedures.",
        "Employment does not grant access to a household’s medical, academic, shopping, or debt records. Essential services must not become an informal instrument of workplace discipline.",
    ),
];
#[component]
pub fn Governance() -> impl IntoView {
    let active = RwSignal::new(1usize);
    view! {<section class="page governance-page"><PageTitle kicker="05 / AUTHORITY & MEMBER RIGHTS" title="Integrated infrastructure. Independent people." description="A durable ecosystem makes decisions clearly, protects private records, and remains accountable when people disagree." index="05"/>
        <div class="governance-layout"><div class="authority-menu">{AUTHORITIES.iter().enumerate().map(|(i,(label,org,_,_,_))|view!{<button class="authority-card panel" class:active=move ||active.get()==i on:click=move |_|active.set(i)><span>{format!("0{}",i+1)}</span><div><b>{*label}</b><small>{*org}</small></div><i>"↗"</i></button>}).collect_view()}</div><div class="authority-detail panel">{move ||{let(label,org,title,body,boundary)=AUTHORITIES[active.get()];view!{<p class="eyebrow">{format!("{} AUTHORITY / {}",label.to_uppercase(),org.to_uppercase())}</p><h2>{title}</h2><p class="lead-paragraph">{body}</p><div class="callout"><span>"THE BOUNDARY"</span><p>{boundary}</p></div><div class="rights-strip"><span>"VOLUNTARY ACCESS"</span><span>"PRIVATE RECORDS"</span><span>"INDEPENDENT APPEAL"</span><span>"ORDERLY EXIT"</span></div><div class="governance-flow"><span>"A defined mandate"</span><b>"→"</b><span>"Authorized action"</span><b>"→"</b><span>"Reviewable outcome"</span></div>}}}</div></div>
    </section>}
}

const STAGES: [(&str, &str, &str, &str, &str); 6] = [
    (
        "00",
        "Define the system",
        "Blueprint & boundaries",
        "Define the complete service catalogue, resource boundary, production chains, institutional authority, and open dependencies. Design the full map before selecting the first deployment.",
        "Traceable specifications, explicit unknowns, and a clear definition of the independence claim.",
    ),
    (
        "01",
        "Qualify the service",
        "Accountable delivery",
        "Provide a defined service through a responsible operator, qualified staff, accepted quality, actual funding, and a practical continuity plan. External supply remains visible.",
        "A complete supported journey, accepted by the responsible operator and relevant professionals.",
    ),
    (
        "02",
        "Control operations",
        "Owned productive assets",
        "Commission the facilities and equipment that deliver the service. Measure output, input consumption, maintenance, quality, cost, and losses against the actual service promise.",
        "Operating evidence at declared demand, with named asset responsibility and dependable maintenance.",
    ),
    (
        "03",
        "Produce the inputs",
        "Recurring supply independence",
        "Replace selected recurring purchases with qualified internal production. Trace ingredients and components upstream and test representative seasons and operating conditions.",
        "Accepted internal output at required quantity and quality, with every remaining external input disclosed.",
    ),
    (
        "04",
        "Reproduce capacity",
        "Repair & replacement",
        "Demonstrate the parts, tools, materials, skills, and manufacturing processes needed to repair and replace critical productive assets. Follow the equipment that makes the equipment.",
        "Qualified replacement pathways, production capacity, calibration, and evidence for longer-lived assets.",
    ),
    (
        "05",
        "Demonstrate independence",
        "Verified for a stated scope",
        "Reconcile the whole declared system: service quality, demand, resource inputs, stocks, losses, equipment replacement, and continuity. Retain exclusions and reopen acceptance when conditions change.",
        "Independent review of operating trials and engineering evidence. No claim of indefinite closure from a short demonstration.",
    ),
];
#[component]
pub fn Roadmap() -> impl IntoView {
    let deployment = RwSignal::new(true);
    let active = RwSignal::new(0usize);
    view! {<section class="page roadmap-page"><PageTitle kicker="06 / THE PATH TO INDEPENDENCE" title="Earn every step toward self-reliance." description="A progression of evidence—not a promise that the entire ecosystem can be built on one launch date." index="06"/>
        <div class="section-switcher"><button class:active=move ||deployment.get() on:click=move |_|deployment.set(true)>"First deployment"</button><button class:active=move ||!deployment.get() on:click=move |_|deployment.set(false)>"Path to independence"</button></div>
        {move ||if deployment.get(){view!{<crate::readiness::Deployment/>}.into_any()}else{view!{<div class="roadmap-stages">
        <div class="stage-track">{STAGES.iter().enumerate().map(|(i,(n,title,_,_,_))|view!{<button class:active=move ||active.get()==i on:click=move |_|active.set(i)><span>{*n}</span><b>{*title}</b><i/></button>}).collect_view()}</div>
        <div class="roadmap-detail panel">{move ||{let(n,title,sub,body,proof)=STAGES[active.get()];view!{<div class="roadmap-number"><span>"EVIDENCE STAGE"</span><b>{n}</b><small>"PROPOSED FRAMEWORK"</small></div><div class="roadmap-copy"><p class="eyebrow">{sub.to_uppercase()}</p><h2>{title}</h2><p class="lead-paragraph">{body}</p><div class="callout"><span>"EVIDENCE TO ADVANCE"</span><p>{proof}</p></div></div><div class="roadmap-principles"><h3>"WHAT STAYS VISIBLE"</h3><Bullets items=vec!["Remaining external inputs".into(),"Real capacity and quality".into(),"Replacement obligations".into(),"Member continuity and rights".into()]/></div>}}}</div>
        </div>}.into_any()}}
        <div class="roadmap-bottom"><div><span class="status-dot"/><p>"Current baseline: target architecture. Facilities, budgets, production yields, and independence claims require separate evidence."</p></div><a class="outline-button" href="#/atlas">"Return to the ecosystem"<Arrow/></a></div>
    </section>}
}

#[component]
pub fn Systems() -> impl IntoView {
    let filter = RwSignal::new("All domains");
    let search = RwSignal::new(String::new());
    let page = RwSignal::new(0usize);
    let filtered = Memo::new(move |_| {
        DOMAINS
            .iter()
            .filter(|d| {
                (filter.get() == "All domains" || d.sector == filter.get())
                    && (search.get().is_empty()
                        || format!("{} {} {}", d.name, d.summary, d.tagline)
                            .to_lowercase()
                            .contains(&search.get().to_lowercase()))
            })
            .cloned()
            .collect::<Vec<_>>()
    });
    view! {<section class="page systems-page"><PageTitle kicker="02 / THE OPERATING DOMAINS" title="Every essential. An accountable system." description="Twenty-four domains, each with a purpose, inputs, operating model, and a test of independence." index="02"/>
        <div class="directory-toolbar"><div class="filter-tabs">{["All domains","Living systems","Production","Foundations","Institutions"].into_iter().map(|f|view!{<button class:active=move ||filter.get()==f on:click=move |_|{filter.set(f);page.set(0);}>{f}</button>}).collect_view()}</div><label class="search-field"><span>"⌕"</span><input aria-label="Search domains" placeholder="Find a domain…" prop:value=move ||search.get() on:input=move |ev|{search.set(event_target_value(&ev));page.set(0);}/></label></div>
        <div class="domain-grid">{move ||{let items=filtered.get();if items.is_empty(){view!{<div class="empty-state"><h2>"No matching domain."</h2><p>"Try food, energy, housing, or a different search."</p></div>}.into_any()}else{items.into_iter().skip(page.get()*6).take(6).map(|d|view!{<a class="domain-card" href=format!("#/domain/{}",d.id)><div class="card-top"><Icon name=d.id.clone()/><span>{format!("{} / {}",number(&d.id),d.sector.to_uppercase())}</span><span class="card-arrow">"↗"</span></div><h2>{d.name.clone()}</h2><p>{d.tagline.clone()}</p><div class="card-bottom"><span>"VIEW OPERATING BLUEPRINT"</span><span>"→"</span></div></a>}).collect_view().into_any()}}}</div>
        <div class="pagination"><span>{move ||format!("{} DOMAINS IN VIEW",filtered.get().len())}</span><div><button class="text-button" disabled=move ||page.get()==0 on:click=move |_|page.update(|p|*p=p.saturating_sub(1))>"← Previous"</button><span>{move ||format!("{:02} / {:02}",page.get()+1,filtered.get().len().div_ceil(6).max(1))}</span><button class="text-button" disabled={move ||(page.get()+1)*6>=filtered.get().len()} on:click=move |_|page.update(|p|*p+=1)>"Next →"</button></div></div>
    </section>}
}

#[component]
pub fn DomainPage(id: String) -> impl IntoView {
    let d = domain(&id);
    let tab = RwSignal::new(0usize);
    view! {<section class="page domain-page"><div class="domain-breadcrumb"><a href="#/systems">"← All domains"</a><span>"/"</span><span>{d.sector.clone()}</span><span>"/"</span><b>{number(&d.id)}</b></div><div class="domain-title"><div><p class="eyebrow">{format!("OPERATING BLUEPRINT / {}",d.sector.to_uppercase())}</p><h1>{d.name.clone()}</h1><p>{d.tagline.clone()}</p></div><Icon name=d.id.clone()/></div>
        <div class="domain-layout"><aside class="domain-visual panel"><div class="technical-label">"PART OF A CONNECTED WORLD"</div><div class="domain-visual-art"><img src="public/atlas.svg" alt="The connected physical ecosystem"/><div class="domain-emblem"><Icon name=d.id.clone()/></div></div><p class="technical-label">"CONNECTED DOMAINS"</p><div class="dependency-links">{d.links.iter().map(|id|{let linked=domain(id);view!{<a href=format!("#/domain/{id}")><span>{linked.name.clone()}</span><span>"↗"</span></a>}}).collect_view()}</div><a class="back-atlas" href=if d.id=="education"{"#/learning"}else if d.id=="governance"{"#/place/community-center"}else{"#/community"}>"Explore the family-life connection →"</a>{(d.id=="recovery").then(||view!{<a class="back-atlas" href="#/recovery">"Explore the recovery campus & material loops →"</a>})}<a class="back-atlas" href="#/atlas">"Return to the physical atlas →"</a></aside>
        <div class="domain-reading panel"><div class="tabs" role="tablist" aria-label="Domain chapters">{["Overview","How it operates","Supply & replacement","Evidence required","Readiness"].into_iter().enumerate().map(|(i,label)|view!{<button role="tab" aria-selected=move ||(tab.get()==i).to_string() class:active=move ||tab.get()==i on:click=move |_|tab.set(i)>{label}</button>}).collect_view()}</div><div class="chapter" role="tabpanel">{move ||match tab.get(){
            0=>view!{<p class="eyebrow">"THE PURPOSE"</p><h2>{d.tagline.clone()}</h2><p class="lead-paragraph">{d.summary.clone()}</p><div class="two-columns"><div><h3>"REQUIRED INPUTS"</h3><Bullets items=d.inputs.clone()/></div><div><h3>"DELIVERED OUTPUTS"</h3><Bullets items=d.outputs.clone()/></div></div>}.into_any(),
            1=>view!{<p class="eyebrow">"THE OPERATING MODEL"</p><h2>"From intent to dependable delivery."</h2><div class="operating-steps">{d.build.iter().enumerate().map(|(i,text)|view!{<div><b>{format!("0{}",i+1)}</b><p>{text.clone()}</p></div>}).collect_view()}</div><div class="callout"><span>"ACCOUNTABILITY"</span><p>"A named operator owns the service promise, staffing, quality, maintenance, and continuity. Shared technology supports that responsibility."</p></div>}.into_any(),
            2=>view!{<p class="eyebrow">"THE INDEPENDENCE QUESTION"</p><h2>"What keeps this system running?"</h2><p class="lead-paragraph">{d.replacement.clone()}</p><div class="two-columns"><div><h3>"TRACE EVERY INPUT"</h3><Bullets items=d.inputs.clone()/></div><div><h3>"FOLLOW THE NEXT CONNECTION"</h3><div class="dependency-links">{d.links.iter().map(|id|view!{<a href=format!("#/domain/{id}")>{domain(id).name.clone()}<span>"↗"</span></a>}).collect_view()}</div></div></div><p class="small-note">"Starting inventory provides a buffer. A qualified internal production pathway closes a recurring dependency."</p>}.into_any(),
            3=>view!{<p class="eyebrow">"THE EVIDENCE GATE"</p><h2>"Define the claim. Demonstrate it."</h2><p class="lead-paragraph">{d.proof.clone()}</p><div class="evidence-grid">{["Accepted quality","Required capacity","Named responsibility","Measured inputs","Recovery demonstrated","Replacement pathway"].into_iter().enumerate().map(|(i,t)|view!{<div><span>{format!("0{}",i+1)}</span>{t}</div>}).collect_view()}</div><div class="callout"><span>"CURRENT STATUS / PROPOSED"</span><p>"This digital blueprint describes the target architecture. It does not claim that these facilities, capacities, or independence tests have been completed."</p></div>}.into_any(),
            _=>view!{<p class="eyebrow">"SEPARATE EVIDENCE / SEPARATE CLAIMS"</p><h2>"Readiness by capability."</h2><p class="lead-paragraph">"Each dimension advances on its own evidence. A working demo does not establish an operating service or commissioned infrastructure."</p><crate::readiness::Readiness/>}.into_any()
        }}</div></div></div>
    </section>}
}
