# Community costs and choice — model notes

This feature implements the supplied `Saber_Tech_Community_Costs_Codex_Plan.txt`, with the founder's subsequent clarification that the proposed $50 million annual operating total includes housing, healthcare, utilities, school, community centers, parks, and other community operations for 10,000 residents. These are hypothetical planning inputs, not independently verified costs, current prices, service availability, or commitments.

## Budget boundary

| Original annual allocation | Amount | Treatment |
| --- | ---: | --- |
| School | $10,000,000 | Full school allocation, including school overhead and shared-cost allocation |
| Hospital | $11,000,000 | Inside non-school operations; no healthcare opt-out price is inferred |
| Remaining community operations | $29,000,000 | Unitemized; includes housing, utilities, community centers, parks, and other operations |
| Total | $50,000,000 | Each expense appears once |

This annual operating scenario does not establish construction financing or an individual healthcare package. Actual service scopes, itemized budgets, supporting evidence, and participation terms are future work. The overview remains the original founder scenario; editable calculator assumptions are separate and do not invent allocations within the remainder.

## Allocation contract

All money inputs are USD. Defaults are total budget B=$50m, school S=$10m, resident count N=10,000, enrolled resident students E=2,000, annual support from other sources D=0, household residents H=2, household enrolled students K=0, optional household monthly support G=0, and an absent alternative school price F.

- Other operating cost = B − S. Its monthly per-resident requirement = (B − S) / (12 × N).
- Annual school support = D + 12 × G. D explicitly excludes the displayed household's support.
- Support applied = min(S, support). Excess remains an unallocated surplus; it never lowers other-service charges.
- Remaining school requirement = max(0, S − support). With enrollment E > 0, divide this by 12 × E for the required monthly student rate. With E = 0, the rate is unavailable and the remaining need stays visible.
- Household total = H × other rate + K × school rate + G; the school term is zero when K = 0.
- The household's displayed monthly per-resident average is its exact household total divided by H. Its annual breakdown multiplies each exact monthly component by 12 before rounding. Household enrollment updates these household figures; it does not change the independent communitywide enrollment assumption or operating budget.
- The optional comparison uses entered price F: annual student charges = 12 × E × F. Add assumed support to show the gap or surplus against S. Its separate household total uses K × F. It never replaces the primary required rate.
- Annual other-service requirements + school participant requirements + applied support reconcile to B. At zero enrollment, the unfilled school requirement replaces participant charges. Surplus is additional and separate.

The model treats support as a funding allocation, not an expense reduction. Resident-funded support and outside support have different effects on collective resident spending; their source split is not calculated here. General school support benefits all enrolled students, rather than being targeted to a chosen household. No school enrollment in a household gives zero required school participation in the calculator, while the wider eligibility policy remains undecided.

## Numerical behavior

Inputs are parsed into integer cents with at most two decimal places; no floating-point money arithmetic or locale-dependent currency parsing is used. Counts are whole numbers. Each money field is limited to $1 trillion and each count to 1 billion; checked integer operations protect intermediate arithmetic. Exact rational rates are retained through household totals; only final display rounds half-up to cents.

Require N > 0; 0 ≤ S ≤ B; 0 ≤ E ≤ N; 1 ≤ H ≤ N; 0 ≤ K ≤ H and K ≤ E; E − K ≤ N − H. These rules prevent counting more students outside the household than residents outside it. Negative, empty required, malformed, excessive-precision, and out-of-range values remain in their fields with explanations. Invalid base inputs remove the primary estimate. An invalid comparison price only disables the comparison. A blank comparison is inactive and differs from explicitly testing $0.

Displayed rounded lines can differ from their total by a cent. For four residents and two students, household other services are displayed as $1,333.33 and school as $833.33; the exact total rounds to $2,166.67. The annual total is $26,000.00, computed before monthly display rounding. With a $50 monthly household gift, its $600 annual support reduces the shared school rate and the household total becomes $2,216.62 monthly or $26,599.40 annually.

## Interface and evidence

`#/costs` has three focused views: How it works, Household calculator, and Funding & accountability. Each has an intentional desktop scroll region; mobile stacks content in reading order. Native details controls expose community assumptions, reconciliation, and the alternative-price comparison. Labels link help and errors to fields. Results update without moving focus; a polite summary is announced when an edit is committed rather than on every keystroke. Reset restores all input defaults and clears errors. Screen-reader audio output has not been independently audited.

The results show an expanded household annual breakdown with enrolled-child count, other services, required school contribution, optional support, and total. The whole-community reconciliation and its pre-allocation average sit in a separately labeled expandable section. The visitor's expanded/collapsed choices persist through edits and invalid-input recovery, so the breakdown can be watched while changing enrollment.

The feature creates no bill, enrollment, coverage, pledge, investment, voting action, or payment. Values exist only in component memory, and are not transmitted, persisted, or placed in a shared URL. Development donations and investment-interest inquiries are separate from hypothetical school support. Optional support creates no extra community voting rights.

The two companion JSON files named in the handoff were not available. Tests were authored from the written calculation contract and reference examples; they are not represented as execution of supplied JSON fixtures. See `VERIFICATION.md` for actual checks.

## Open operating decisions

- Evidenced budgets, location, accountable operators, capacity, and service readiness.
- Individual non-school service choices and infrastructure obligations associated with a home or service.
- School allocation methods, eligibility, schooling elsewhere, participation periods, and withdrawal rules.
- Healthcare scope, funding terms, and individual coverage.
- Minimum viable participation and funding commitments, contribution changes, shortfall procedures, and hardship assistance.
- Handling surplus support, including any reserve, refund, or spending policy.
- Community membership, voting procedures, and management review and replacement authority.

A future service record should identify its stable ID, operator, scope and period, evidenced budget and status, participation unit, count and capacity, required contributions, support sources and restrictions, fixed and variable costs, participation terms, and readiness evidence. This initial two-pool calculator does not fabricate those records for every operating domain.
