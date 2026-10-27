use crate::cost_model::{self as model, Evaluation, Raw, money};
use crate::ui::{Arrow, Icon};
use leptos::prelude::*;

fn announcement(ev: &Evaluation) -> String {
    match &ev.result {
        Ok(r) => format!(
            "Illustrative monthly household contribution: {}. No bill or commitment.",
            r.household_total.usd()
        ),
        Err(_) => "Results unavailable. Review the highlighted inputs.".into(),
    }
}

#[component]
fn CostField(
    index: usize,
    label: &'static str,
    help: &'static str,
    raw: RwSignal<Raw>,
    evaluation: Memo<Evaluation>,
    announced: RwSignal<String>,
) -> impl IntoView {
    let error = move || {
        let ev = evaluation.get();
        if index == 8 {
            ev.comparison.err().unwrap_or_default()
        } else {
            ev.result
                .err()
                .unwrap_or_default()
                .into_iter()
                .filter(|e| e.field == index)
                .map(|e| e.message)
                .collect::<Vec<_>>()
                .join(" ")
        }
    };
    view! {
        <div class="cost-field">
            <label for=format!("cost-{index}")>{label}</label>
            <input id=format!("cost-{index}") type="text" inputmode=if matches!(index,2|3|5|6){"numeric"}else{"decimal"}
                autocomplete="off" spellcheck="false" prop:value=move || raw.get()[index].clone()
                aria-invalid=move || (!error().is_empty()).to_string()
                aria-describedby=format!("cost-help-{index} cost-error-{index}")
                on:input=move |ev| raw.update(|a| a[index]=event_target_value(&ev))
                on:change=move |_| announced.set(announcement(&evaluation.get()))/>
            <p class="cost-help" id=format!("cost-help-{index}")>{help}</p>
            <p class="cost-error" id=format!("cost-error-{index}") hidden=move || error().is_empty()>{error}</p>
        </div>
    }
}

#[component]
fn CostResults(evaluation: Memo<Evaluation>) -> impl IntoView {
    view! { <aside class="cost-results panel" aria-label="Household calculation results">
        {move || match evaluation.get().result {
            Err(_) => view!{<div class="cost-unavailable"><span class="technical-label">"INPUTS NEED ATTENTION"</span><h2>"Results unavailable"</h2><p>"Check the highlighted fields. Your entries are kept; the estimate returns when the community and household assumptions are valid."</p></div>}.into_any(),
            Ok(r) => {
                let no_students=r.input.e==0;
                let zero_school=r.input.s==0;
                view! {
                    <span class="technical-label">"YOUR HOUSEHOLD / MONTHLY / USD"</span>
                    <p class="cost-total" data-testid="household-total">{r.household_total.usd()}</p>
                    <h2>"Estimated monthly household contribution in this scenario"</h2>
                    <dl class="cost-lines">
                        <div><dt>"Your household’s other services"<small>{format!("{} residents · includes housing, healthcare, utilities & shared places",r.input.h)}</small></dt><dd data-testid="household-other">{r.household_other.usd()}</dd></div>
                        <div><dt>"Your household’s school participation"<small>{format!("{} enrolled students",r.input.k)}</small></dt><dd data-testid="household-school">{r.household_school.usd()}</dd></div>
                        <div><dt>"Your household’s optional school support"<small>"Voluntary spending · counted once in the school pool"</small></dt><dd data-testid="household-gift">{money(r.input.g)}</dd></div>
                    </dl>
                    <div class="cost-rate-grid">
                        <div><span>"Other services per resident / month"</span><b data-testid="other-rate">{r.other_rate.usd()}</b></div>
                        <div><span>"School contribution needed per enrolled student / month"</span><b data-testid="school-rate">{r.school_rate.map(|v|v.usd()).unwrap_or("Unavailable".into())}</b></div>
                    </div>
                    <p class="cost-comparison-average"><strong>{r.average.usd()}</strong>" / resident / month is the whole-budget average before dividing costs by participation. It is a comparison, not a household price."</p>
                    <p class="cost-callout" hidden=!no_students data-testid="zero-enrollment">{if r.remaining>0 {format!("No students enrolled in this scenario. {} of the annual school budget remains unfunded. A per-student rate is unavailable.",money(r.remaining))} else {"Assumed support covers the modeled school budget, but there are no enrolled students in this scenario. This does not establish an operating school or guarantee free enrollment.".into()}}</p>
                    <p class="cost-callout" hidden={!zero_school}>"A zero school budget defines no school service scope. All figures remain planning assumptions."</p>
                    <p class="cost-callout" hidden={r.input.b != 0}>"A zero operating budget is an arithmetic scenario; it does not establish that community services cost nothing."</p>
                    <p class="cost-callout" hidden={r.surplus==0} data-testid="support-surplus">{format!("{} annual school-support surplus is unallocated. It is not transferred to other services; refund, reserve, and spending rules are undecided.",money(r.surplus))}</p>
                    <details class="cost-details"><summary>"See the annual budget reconciliation"</summary>
                        <dl class="cost-lines compact">
                            <div><dt>"Other-service contribution requirement"</dt><dd>{money(r.other_annual)}</dd></div>
                            <div><dt>{if no_students {"Unfunded school requirement"}else{"Required school contributions"}}</dt><dd>{money(r.remaining)}</dd></div>
                            <div><dt>"School support applied"</dt><dd>{money(r.applied)}</dd></div>
                            <div class="cost-line-total"><dt>"Total annual operating budget"</dt><dd>{money(r.input.b)}</dd></div>
                        </dl>
                        <p class="cost-help">{format!("Assumed school support: {} from everyone else + {} from your household over 12 months = {}. Any surplus is additional to the budget and shown separately.",money(r.input.d),money(r.input.g*12),money(r.support))}</p>
                        <p class="cost-help">"These are modeled requirements and assumptions, not receipts or confirmed commitments. Support shifts who funds the expense; it does not reduce the operating budget."</p>
                    </details>
                    <p class="cost-help cost-rounding">"This is an illustration, not a bill or a commitment. Figures are rounded; totals use amounts before rounding, so displayed lines may differ from their total by a cent."</p>
                }.into_any()
            }
        }}
    </aside> }
}

#[component]
pub fn CommunityCosts() -> impl IntoView {
    let tab = RwSignal::new(0usize);
    let raw = RwSignal::new(model::defaults());
    let evaluation = Memo::new(move |_| model::evaluate(&raw.get()));
    let announced = RwSignal::new(String::new());
    view! {
    <section class="page costs-page">
        <div class="cost-heading"><div><p class="eyebrow"><span/>"SABER TECH / COMMUNITY ECONOMICS"</p><h1>"Understand the cost."<em>"Choose your participation."</em></h1></div><span class="cost-stage">"PROPOSED MODEL"<i class="status-dot"/></span></div>
        <p class="cost-status"><b>"Planning example."</b>" The $50 million budget illustrates the proposed community. Detailed operating budgets, participation terms, and service availability have not been established."</p>
        <nav class="cost-tabs" aria-label="Costs and choice views">
            {[(0,"01","How it works"),(1,"02","Household calculator"),(2,"03","Funding & accountability")].into_iter().map(|(i,n,label)|view!{
                <button type="button" class:active=move ||tab.get()==i aria-pressed=move ||(tab.get()==i).to_string() aria-controls=format!("cost-view-{i}") on:click=move |_|tab.set(i)><span>{n}</span>{label}</button>
            }).collect_view()}
        </nav>
        <div class="cost-content" tabindex="0" aria-label="Community costs explanation and calculator">
            <section id="cost-view-0" class="cost-overview" hidden=move ||tab.get()!=0 aria-label="How community costs could work">
                <div class="cost-overview-grid">
                    <div class="cost-budget panel">
                        <div class="cost-budget-heading"><span class="technical-label">"ONE INCLUSIVE ANNUAL OPERATING BUDGET"</span><span class="small-badge">"FOUNDER’S PLANNING EXAMPLE"</span></div>
                        <div class="cost-budget-visual">
                            <div class="cost-donut" role="img" aria-label="Annual budget: school 10 million dollars, hospital 11 million dollars, remaining operations 29 million dollars; total 50 million dollars."><div><small>"ANNUAL OPERATIONS"</small><strong>"$50"<span>"M"</span></strong><p>"10,000 residents"</p></div></div>
                            <div class="cost-budget-legend">
                                <div><i class="school"/><span><b>"School"</b><small>"20% · complete school allocation"</small></span><strong>"$10M"</strong></div>
                                <div><i class="hospital"/><span><b>"Hospital"</b><small>"22% · inside non-school operations"</small></span><strong>"$11M"</strong></div>
                                <div><i class="remaining"/><span><b>"Remaining operations"</b><small>"58% · individual allocations pending"</small></span><strong>"$29M"</strong></div>
                                <p>"The hospital and remaining operations form the $40M non-school pool. Every expense is counted once."</p>
                            </div>
                        </div>
                        <div class="cost-average-strip"><strong>"$416.67"</strong><span>"Average per resident / month"<small>"$5,000 annually · before allocating costs by participation"</small></span></div>
                    </div>
                    <div class="cost-scope panel">
                        <span class="technical-label">"WHAT THE TOTAL IS INTENDED TO OPERATE"</span><h2>"The places we live."<br/>"The services we rely on."</h2>
                        <div class="cost-inclusions">{[("housing","Housing"),("health","Healthcare"),("energy","Utilities"),("education","School"),("community","Community centers"),("play","Parks & shared places")].into_iter().map(|(icon,label)|view!{<div><Icon name=icon.to_string()/><span>{label}</span></div>}).collect_view()}</div>
                        <p>"The $50M is the proposed annual cost of running the whole community, including these services and other operations. Housing, utilities, parks, and community centers sit within the unitemized remainder; their individual budgets are not yet established."</p>
                        <p class="cost-help">"This operating example does not establish construction financing, individual healthcare coverage, or final service terms."</p>
                        <button class="primary-button" type="button" on:click=move |_|tab.set(1)>"Explore a household example"<Arrow/></button>
                    </div>
                </div>
                <div class="cost-principles">
                    <article><span>"01 / PARTICIPATE"</span><h3>"Use a service."</h3><p>"See its scope, budget, and contribution method before agreeing to participate. Families using the school share its participant-funded requirement."</p></article>
                    <article><span>"02 / SHARE"</span><h3>"Know what supports your home."</h3><p>"Understand the infrastructure, maintenance, and availability your home and services rely on. Final cost-allocation and participation rules remain open."</p></article>
                    <article><span>"03 / SUPPORT"</span><h3>"Help because you choose to."</h3><p>"People without children would not be required to fund the school. They could choose to support education voluntarily. Optional support starts at zero."</p></article>
                </div>
                <p class="cost-overview-note">"The calculator explores school participation first. All 10,000 residents participate in the grouped non-school services in this example; the $333.33 allocation is not an adopted mandatory base fee."</p>
            </section>

            <section id="cost-view-1" hidden=move ||tab.get()!=1 aria-label="Household calculator">
                <div class="cost-calculator-intro"><div><h2>"Explore a household example."</h2><p>"Housing, healthcare, utilities, school, community centers, and parks are already inside the total. Change the assumptions to see how the same costs are shared."</p></div><button type="button" class="outline-button" on:click=move |_|{raw.set(model::defaults());announced.set("Example reset. Two household residents, no enrolled students, and no optional support. Monthly estimate: $666.67.".into());}>"Reset example"<span aria-hidden="true">"↺"</span></button></div>
                <div class="cost-calculator-grid">
                    <div class="cost-controls panel">
                        <fieldset><legend>"Your household"</legend><p class="cost-help">"Everyone in the household counts, including children. USD amounts; no login or personal information needed."</p>
                            <CostField index=5 label="Residents in your household" help="Includes adults and children. Changing this does not change the community total." raw evaluation announced/>
                            <CostField index=6 label="Children enrolled in this school" help="Part of your household, not additional residents. Community enrollment stays independent." raw evaluation announced/>
                            <CostField index=7 label="Your optional monthly school support (USD)" help="Starts at $0. Counted once in the school pool and once in your spending; this records no gift or pledge." raw evaluation announced/>
                        </fieldset>
                        <p class="cost-callout">"No enrolled children means no required school contribution in this calculator. This enrollment-based assumption is provisional; eligibility and treatment of schooling elsewhere remain undecided."</p>
                        <details class="cost-details cost-assumptions"><summary>"Edit community assumptions"<span>"Annual budgets & participation"</span></summary>
                            <div class="cost-community-fields">
                                <CostField index=0 label="Total annual operating budget (USD)" help="Includes the school and all other modeled operations. Maximum $1 trillion per money field." raw evaluation announced/>
                                <CostField index=1 label="Annual school budget included in total (USD)" help="Complete school allocation, including its share of common costs. No added school surcharge." raw evaluation announced/>
                                <CostField index=2 label="Residents in the community" help="Children included. All participate in other services here. Maximum 1 billion per count field." raw evaluation announced/>
                                <CostField index=3 label="Students enrolled in the school" help="2,000 is illustrative, not confirmed enrollment. All students are residents in this example." raw evaluation announced/>
                                <CostField index=4 label="Annual school support from everyone else (USD)" help="Excludes your household’s optional support. No actual support or commitments are asserted." raw evaluation announced/>
                            </div>
                        </details>
                        <p class="cost-help">"Fewer students sharing the same school budget need to cover more per student. Assumed support reduces the participant requirement, not the operating expense. General school support benefits all enrolled students; targeted hardship assistance needs separate rules."</p>
                    </div>
                    <CostResults evaluation/>
                </div>
                <details class="cost-details cost-alternative panel"><summary>"Would a different school contribution cover the budget?"<span>"Optional comparison · separate from the required rate"</span></summary>
                    <div class="cost-alternative-grid"><CostField index=8 label="Monthly school contribution to test per student (USD)" help="Leave blank to keep the comparison inactive. Entering 0 explicitly tests a zero-dollar contribution." raw evaluation announced/>
                        <div>{move ||match evaluation.get().comparison {
                            Err(_) => view!{<p class="cost-callout">"Comparison unavailable. Correct the test amount; the primary calculation stays independent."</p>}.into_any(),
                            Ok(None) => view!{<p class="cost-help">{if evaluation.get().result.is_err(){"Correct the household and community assumptions to compare a price."}else{"Enter a monthly school amount to reveal annual contributions, the funding gap or surplus, and the alternative household total."}}</p>}.into_any(),
                            Ok(Some(c)) => view!{<dl class="cost-lines compact">
                                <div><dt>"Modeled annual school contributions"</dt><dd>{money(c.fees)}</dd></div>
                                <div><dt>"Contributions + assumed annual support"</dt><dd>{money(c.funding)}</dd></div>
                                <div class="cost-gap"><dt>"Remaining annual funding gap"</dt><dd data-testid="comparison-gap">{money(c.gap)}</dd></div>
                                <div><dt>"Unallocated annual surplus"</dt><dd data-testid="comparison-surplus">{money(c.surplus)}</dd></div>
                                <div><dt>"Alternative household total / month"</dt><dd data-testid="comparison-total">{c.household_total.usd()}</dd></div>
                            </dl>}.into_any()
                        }}</div>
                    </div><p class="cost-help">"A proposed price does not establish funding. The required school rate above stays visible and unchanged. Any gap needs an explicit decision; it is never silently charged to nonparticipants."</p>
                </details>
                <p class="cost-browser-note">"An educational model running in your browser. No enrollment, bills, money, pledges, or coverage are created. Values are not saved, included in shared links, or sent to the donation or interest forms."</p>
                <p class="sr-only" role="status" aria-live="polite" aria-atomic="true">{move ||announced.get()}</p>
            </section>

            <section id="cost-view-2" class="cost-accountability" hidden=move ||tab.get()!=2 aria-label="Funding and accountability">
                <div class="cost-account-intro"><span class="technical-label">"A MODEL TO TEST AND DEVELOP"</span><h2>"People should see what their contribution does."</h2><p>"The proposed process gives residents meaningful choice and operators a defined funding period. A calculated break-even amount is a requirement to test, not evidence that a service is funded or ready."</p></div>
                <div class="cost-funding-steps">{[("01","Publish the budget","Explain scope, full costs, shared facilities, participation units, and the responsible operator."),("02","Agree to participate","Make contributions, the service period, and withdrawal and change terms clear before agreement."),("03","Establish enough funding","Test participation, capacity, and commitments before undertaking to deliver the service."),("04","Report and review","Show how funds are used and make proposed scope or contribution changes visible before the next period.")].into_iter().map(|(n,title,copy)|view!{<article class="panel"><span>{n}</span><h3>{title}</h3><p>{copy}</p></article>}).collect_view()}</div>
                <div class="cost-account-grid">
                    <article class="panel"><h3>"When funding falls short"</h3><p>"Show the gap. Future choices could include revising scope or timing, increasing enrollment within capacity, finding willing support, or proposing a different contribution to participants."</p><p>"Do not assume gifts will arrive or charge people outside the service. Fixed costs do not disappear when enrollment falls. School costs, including their share of common overhead, must stay in the school allocation."</p></article>
                    <article class="panel"><h3>"A voice in the community"</h3><p>"Community management should answer to the people living and participating here. The intended charter would define their role in choosing, reviewing, and replacing management."</p><p>"Optional support does not buy extra community voting power. Community membership, management authority, voting procedures, and any corporate shareholder rights need their own clear rules."</p></article>
                </div>
                <details class="cost-details panel cost-open-decisions"><summary>"Decisions still to establish"</summary><ul><li>"Evidenced operating budgets, selected development, accountable operators, and capacity."</li><li>"Independent selection of non-school services and infrastructure obligations attached to a home."</li><li>"School eligibility, allocation methods, enrollment elsewhere, participation periods, and withdrawal rules."</li><li>"Healthcare scope and funding terms; a hospital budget alone does not establish individual coverage."</li><li>"Hardship assistance, contribution changes, funding shortfalls, and treatment of excess support."</li><li>"Governance membership, voting procedures, and authority to replace management."</li></ul></details>
                <div class="cost-next"><p>"A working blueprint is the current stage. No launch date, available homes, commissioned hospital, confirmed enrollment, or funded community is asserted."</p><a class="outline-button" href="#/roadmap">"Explore the proposed roadmap"<Arrow/></a><a class="outline-button" href="#/governance">"How community decisions would work"<Arrow/></a></div>
                <p class="cost-browser-note">"Current project donations support Saber’s development. They do not pay for these modeled services or create school enrollment, healthcare coverage, membership, voting rights, shares, or a financial return. Early investment-interest inquiries remain a separate process."</p>
            </section>
        </div>
    </section> }
}
