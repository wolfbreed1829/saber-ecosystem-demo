use crate::ui::*;
use leptos::prelude::*;

#[component]
pub fn Readiness() -> impl IntoView {
    view! {<div class="readiness-grid" aria-label="Domain evidence status">{[("Software","Unverified","A working presentation exists. Domain software requires a versioned implementation, test results, and operator acceptance."),("Service operations","Unverified","No service operating record is published here. Require a named operator, qualified staff, defined scope, and accepted service trial."),("Physical infrastructure","Unverified","No commissioned facility is evidenced here. Require a selected site, asset register, applicable approvals, and commissioning records.")].into_iter().map(|(title,status,body)|view!{<article><div><h3>{title}</h3><span>{status}</span></div><p>{body}</p></article>}).collect_view()}</div>}
}

#[component]
pub fn Deployment() -> impl IntoView {
    let active = RwSignal::new(0usize);
    let items = [
        (
            "Deliver first",
            "A complete household service request",
            "Proposed first slice: one household submits a housing maintenance request, authorizes an access window, follows the assigned work, and accepts or disputes completion. The request crosses resident access, Aether coordination, operator action, and an evidence record.",
        ),
        (
            "Initial users",
            "An opt-in household pilot",
            "A small invited cohort of employees or partner households already served by a participating housing operator. Cohort size, location, and start date remain to be selected. Begin in a sandbox with synthetic records; introduce real households only after operational readiness.",
        ),
        (
            "Accountable operator",
            "One named housing service operator",
            "The housing operator owns triage, qualified assignment, repairs, resident communication, and service recovery. A designated pilot lead coordinates the trial. Specific organization and accountable person have not yet been appointed in the documented strategy.",
        ),
        (
            "Dependencies",
            "A bounded workflow with real responsibilities",
            "Before a live pilot: participating housing stock, clear service terms, identity and scoped roles, repair intake, qualified trades, an escalation path, authorized access, funding, and an exportable activity record. Existing external suppliers remain explicitly identified.",
        ),
        (
            "Success evidence",
            "A resolved request, with the boundary intact",
            "Proposed acceptance gate: every test request reaches a documented resolution or explicit escalation; required permission tests pass; unauthorized roles cannot access the case; the household can review or dispute completion. Measure completion rate, elapsed time, repeat visits, and resident acceptance. Agree numeric service targets before launch.",
        ),
    ];
    view! {<div class="deployment-layout"><div class="deployment-menu panel"><p class="eyebrow">"PROPOSED FIRST DEPLOYMENT"</p><h2>"One useful workflow."<br/>"A complete responsibility chain."</h2>{items.into_iter().enumerate().map(|(i,(label,_,_))|view!{<button class:active=move ||active.get()==i on:click=move |_|active.set(i)><span>{format!("0{}",i+1)}</span>{label}<b>"↗"</b></button>}).collect_view()}</div><article class="deployment-detail panel">{move ||{let(label,title,body)=items[active.get()];view!{<p class="eyebrow">{label.to_uppercase()}</p><h2>{title}</h2><p class="lead-paragraph">{body}</p><div class="pilot-flow"><span>"REQUEST"</span><b>"→"</b><span>"AUTHORIZE"</span><b>"→"</b><span>"DELIVER"</span><b>"→"</b><span>"REVIEW"</span></div><div class="family-note"><span>"WHY THIS STARTING POINT"</span><p>"A useful resident experience can test coordination, scoped access, operator responsibility, and evidence before a new town or full supply chain exists. This is a proposed planning choice, not an announced launch."</p></div><a class="outline-button" href="#/domain/housing">"Inspect housing readiness"<Arrow/></a>}}}</article></div>}
}
