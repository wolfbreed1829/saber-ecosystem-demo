use crate::ui::{Arrow, Icon};
use leptos::prelude::*;

struct MaterialLoop {
    name: &'static str,
    icon: &'static str,
    input: &'static str,
    process: &'static str,
    output: &'static str,
    description: &'static str,
    condition: &'static str,
    destinations: [(&'static str, &'static str); 2],
    source: (&'static str, &'static str),
}
const LOOPS: [MaterialLoop; 7] = [
    MaterialLoop {
        name: "Tires → road materials",
        icon: "logistics",
        input: "Separated end-of-life tires",
        process: "Rubber recovery & pavement qualification",
        output: "Rubber-modified asphalt inputs",
        description: "Route suitable recovered tire rubber into an engineered paving mix for roads and service lanes. The road operator must accept the mix and its performance before use.",
        condition: "Tires become one input to a pavement system. Aggregate, binder, processing, testing, and residual handling still have their own requirements.",
        destinations: [
            ("construction", "Road construction"),
            ("materials", "Materials production"),
        ],
        source: (
            "FHWA · recycled tire rubber in asphalt",
            "https://www.fhwa.dot.gov/pavement/pub_details.cfm?id=1139",
        ),
    },
    MaterialLoop {
        name: "Food scraps → compost",
        icon: "agriculture",
        input: "Separated food scraps & clean organics",
        process: "Managed composting, curing & screening",
        output: "Finished compost for soil",
        description: "Recover unavoidable food scraps from homes, kitchens, and groceries. Compost them with suitable carbon-rich material before returning accepted output to growing areas and landscaping.",
        condition: "Raw or merely shredded food is not finished compost. Contamination, maturity, and suitability for the intended use must be checked.",
        destinations: [
            ("agriculture", "Food-growing systems"),
            ("community", "Gardens & shared places"),
        ],
        source: (
            "EPA · composting",
            "https://www.epa.gov/sustainable-management-food/composting",
        ),
    },
    MaterialLoop {
        name: "Plastics → new material",
        icon: "materials",
        input: "Accepted plastics, separated by resin",
        process: "Sort, clean & mechanically reprocess",
        output: "Qualified recycled plastic feedstock",
        description: "Give suitable packaging and plastic bottles a material-recycling route. Product specifications and feedstock quality determine what can be made from the recovered material.",
        condition: "A plastic-to-fuel or pyrolysis route would be a separate feasibility project with its own outputs, residues, energy use, and acceptance tests. Those products are not biodiesel; no such plant is claimed here.",
        destinations: [
            ("materials", "Material qualification"),
            ("grocery", "Packaging systems"),
        ],
        source: (
            "EPA · mechanical and advanced plastics recycling",
            "https://www.epa.gov/plastics/advanced-recycling-plastics",
        ),
    },
    MaterialLoop {
        name: "Aluminum → new metal",
        icon: "machinery",
        input: "Aluminum cans & compatible scrap",
        process: "Sort by alloy, remelt & form",
        output: "Accepted aluminum stock or products",
        description: "Keep aluminum in the materials loop through sorting, remelting, and fabrication. Qualified output can supply new containers or other compatible products.",
        condition: "Aluminum is a recyclable metal, not a biodiesel feedstock. Alloy quality, processing energy, losses, and accepted product destinations remain visible.",
        destinations: [
            ("materials", "Metals production"),
            ("machinery", "Manufacturing"),
        ],
        source: (
            "Aluminum Association · secondary production",
            "https://www.aluminum.org/secondary-production-101",
        ),
    },
    MaterialLoop {
        name: "Glass bottles → new glass",
        icon: "grocery",
        input: "Accepted glass bottles & containers",
        process: "Separate, clean & process cullet",
        output: "Feedstock for new glass products",
        description: "Give glass containers a separate route into new glass. Plastic bottles follow the plastics loop; a bottle’s material determines its destination.",
        condition: "Qualify accepted container types, contamination limits, and a receiving process. Collection alone does not demonstrate a completed recycling loop.",
        destinations: [
            ("materials", "Glass production"),
            ("grocery", "Container supply"),
        ],
        source: (
            "EPA · common recyclables",
            "https://www.epa.gov/recycle/how-do-i-recycle-common-recyclables",
        ),
    },
    MaterialLoop {
        name: "Cooking oil → biodiesel",
        icon: "fuel",
        input: "Separately collected used cooking oils & fats",
        process: "Qualified biodiesel production & testing",
        output: "Specification-compliant biodiesel",
        description: "Connect kitchen oil recovery to the fuel domain. Biodiesel production uses suitable oils and fats; recovered cooking oil itself is not ready-to-use biodiesel.",
        condition: "Production inputs, fuel-quality evidence, coproduct handling, and compatibility with the intended fleet must be established. This is a proposed pathway, with no yield or fuel-independence claim.",
        destinations: [
            ("fuel", "Fuel production & stations"),
            ("logistics", "Compatible fleet uses"),
        ],
        source: (
            "DOE · biodiesel production",
            "https://afdc.energy.gov/fuels/biodiesel-production",
        ),
    },
    MaterialLoop {
        name: "Yard material → mulch",
        icon: "land",
        input: "Clean woody landscape trimmings",
        process: "Separate, chip & qualify",
        output: "Landscape mulch",
        description: "The organics facility also maintains a clean woody-material stream for mulch. Food scraps take the composting route; the two products are managed separately.",
        condition: "Select appropriate feedstocks and intended uses. Keep contaminated material out and account for rejected material and processing losses.",
        destinations: [
            ("land", "Landscape stewardship"),
            ("community", "Parks & gardens"),
        ],
        source: (
            "New York DEC · organic materials management",
            "https://dec.ny.gov/environmental-protection/recycling-composting/organic-materials-management",
        ),
    },
];

#[component]
pub fn Recovery() -> impl IntoView {
    let view = RwSignal::new(0usize);
    let facility = RwSignal::new(0usize);
    let stream = RwSignal::new(0usize);
    let facilities = [
        (
            "Recycling plant",
            "Recover usable material.",
            "Receive separated recyclables, identify their material, remove contamination, and prepare accepted streams for qualified processing. The intended campus connects sorting with material recovery; each internal processing line still needs a selected design and evidence.",
            "Sorting hall · separated storage · processing areas · quality checks",
            "Measure input mass, accepted recovered output, rejects, processing inputs, and the destination that takes each output.",
        ),
        (
            "Waste management",
            "Every stream has a destination.",
            "Coordinate household collection, commercial pickups, intake, weighing, and residual handling. Keep organics, recyclable material, used cooking oil, and special wastes in their appropriate streams. Collection and disposal responsibilities remain accountable even when a material cannot be recovered.",
            "Collection depot · intake & weighing · transfer bays · residual routing",
            "Record collection coverage, contamination, responsible operators, and confirmed treatment or disposal destinations. Specialized waste pathways remain separate.",
        ),
        (
            "Compost & mulch facility",
            "Return organics to useful work.",
            "Provide a managed food-scrap composting line and a separate clean woody-material mulch stream. Accepted finished products can serve gardens, agriculture, parks, and landscaping after quality checks and an agreed receiving use.",
            "Organics reception · composting & curing · screening · clean mulch storage",
            "Demonstrate controlled processing, mature compost, acceptable contamination, useful output, and management of runoff, odor, and rejects.",
        ),
    ];
    view! {<section class="page recovery-page">
        <div class="recovery-heading"><div><p class="eyebrow"><span/>"SABER TECH / MATERIAL RECOVERY"</p><h1>"A second life."<em>"A connected purpose."</em></h1></div><a class="outline-button" href="#/costs">"$150M community upkeep"<Arrow/></a></div>
        <p class="recovery-status">"PROPOSED INFRASTRUCTURE"<span>"Three connected operations within the annual community plan. Facility budgets, capacity, site, and operating readiness remain to be established."</span></p>
        <nav class="recovery-tabs" aria-label="Recovery blueprint views">{["Facility campus","Material loops","Readiness & evidence"].into_iter().enumerate().map(|(i,t)|view!{<button type="button" class:active=move ||view.get()==i aria-pressed=move ||(view.get()==i).to_string() on:click=move |_|view.set(i)><small>{format!("0{}",i+1)}</small>{t}</button>}).collect_view()}</nav>
        <div class="recovery-content" tabindex="0" aria-label="Recovery blueprint content">
            <section hidden=move ||view.get()!=0>
                <div class="recovery-campus-grid">
                    <div class="recovery-map-panel panel"><div class="recovery-map-label"><span>"RESOURCE-RETURN CAMPUS"</span><small>"CONCEPT · NOT A SURVEYED SITE"</small></div><div class="recovery-map" data-selected=move ||facility.get().to_string() inner_html=include_str!("../public/recovery-campus.svg")/><div class="recovery-facility-controls" aria-label="Select a recovery facility">{facilities.into_iter().enumerate().map(|(i,(name,_,_,_,_))|view!{<button type="button" class:active=move ||facility.get()==i aria-pressed=move ||(facility.get()==i).to_string() on:click=move |_|facility.set(i)><span>{format!("0{}",i+1)}</span>{name}</button>}).collect_view()}</div></div>
                    <article class="recovery-facility-detail panel">{move ||{let(name,title,body,spaces,evidence)=facilities[facility.get()];view!{<p class="eyebrow">{name}</p><h2>{title}</h2><p class="recovery-lead">{body}</p><div class="recovery-note"><h3>"Physical scope"</h3><p>{spaces}</p></div><div class="recovery-note"><h3>"What would demonstrate readiness"</h3><p>{evidence}</p></div><button type="button" class="primary-button" on:click=move |_|view.set(1)>"Follow the material loops"<Arrow/></button>}}}</article>
                </div>
                <div class="recovery-budget-strip"><b>"$150M / YEAR"</b><p>"The proposed community upkeep is divided across 10,000 families: a $1,250 monthly equal-share baseline before school participation and optional support. These facilities are included in the unitemized operating remainder; no added facility surcharge or assumed recycling revenue is shown."</p><a href="#/costs">"Inspect the cost model ↗"</a></div>
            </section>
            <section hidden=move ||view.get()!=1>
                <div class="recovery-loop-grid"><nav class="recovery-streams panel" aria-label="Select a material stream"><span class="technical-label">"SEVEN ROUTES / DISTINCT OUTPUTS"</span>{LOOPS.iter().enumerate().map(|(i,l)|view!{<button type="button" class:active=move ||stream.get()==i aria-pressed=move ||(stream.get()==i).to_string() on:click=move |_|stream.set(i)><Icon name=l.icon.to_string()/><span>{l.name}</span><b>"↗"</b></button>}).collect_view()}</nav>
                <article class="recovery-loop-detail panel">{move ||{let l=&LOOPS[stream.get()];view!{<p class="eyebrow">"PROPOSED MATERIAL PATHWAY"</p><h2>{l.name}</h2><div class="recovery-flow">{[("01 / INPUT",l.input),("02 / PROCESS",l.process),("03 / OUTPUT",l.output)].into_iter().map(|(n,t)|view!{<div><span>{n}</span><p>{t}</p></div>}).collect_view()}</div><p class="recovery-lead">{l.description}</p><div class="recovery-note"><h3>"Before this loop can be claimed"</h3><p>{l.condition}</p></div><div class="recovery-destinations">{l.destinations.into_iter().map(|(id,label)|view!{<a class="outline-button" href=format!("#/domain/{id}")>{label}<Arrow/></a>}).collect_view()}</div><a class="recovery-source" href=l.source.1 target="_blank" rel="noopener noreferrer">{l.source.0}" ↗"</a>}}}</article></div>
                <p class="recovery-footnote">"These sources describe material pathways; they do not establish that Saber has commissioned these facilities. Internal processing and any interim external processor must be identified for every stream. No 100% recovery, production yield, savings, or zero-purchase claim is assumed."</p>
            </section>
            <section hidden=move ||view.get()!=2>
                <div class="recovery-evidence-intro"><p class="eyebrow">"ACCOUNT FOR EVERY INPUT AND OUTPUT"</p><h2>"A loop is complete when its output is accepted."</h2><p>"Collection is the first step. The blueprint needs evidence for processing, quality, a receiving use, and the residuals left behind. Recovery must have a dependable operating budget of its own within the community total."</p></div>
                <div class="recovery-evidence-grid">{[("01","Characterize the material","Identify quantities, contamination, variability, and separate handling needs for each stream."),("02","Define the physical operation","Select the site, equipment, responsible operator, staffing, processing inputs, and required permissions."),("03","Qualify the output","Agree receiving specifications for pavement ingredients, compost, mulch, recycled stock, glass, and fuel."),("04","Reconcile the full process","Record outputs, losses, energy, water, consumables, residues, costs, and actual destinations.")].into_iter().map(|(n,title,body)|view!{<article class="panel"><span>{n}</span><h3>{title}</h3><p>{body}</p></article>}).collect_view()}</div>
                <div class="recovery-note recovery-boundary"><h3>"Three different claims"</h3><p>"Material recycling returns usable material. Composting produces a stable soil amendment. Biodiesel production converts suitable oils and fats into a qualified fuel. Aluminum and plastic are not biodiesel feedstocks; any plastic-derived fuel proposal must be evaluated separately."</p><div class="recovery-sources"><a href="https://afdc.energy.gov/fuels/biodiesel-production" target="_blank" rel="noopener noreferrer">"DOE: biodiesel"</a><a href="https://www.epa.gov/plastics/advanced-recycling-plastics" target="_blank" rel="noopener noreferrer">"EPA: plastics pathways"</a><a href="https://www.epa.gov/sustainable-management-food/composting" target="_blank" rel="noopener noreferrer">"EPA: composting"</a></div></div>
                <div class="recovery-destinations"><a class="outline-button" href="#/domain/recovery">"Recovery operating domain"<Arrow/></a><a class="outline-button" href="#/roadmap">"Development roadmap"<Arrow/></a><a class="outline-button" href="#/atlas">"Back to the physical atlas"<Arrow/></a></div>
            </section>
        </div>
    </section>}
}
