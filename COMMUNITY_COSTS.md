# Community costs and choice - current model

The current planning budget is **$150 million per year for 10,000 residents**. The founder clarified that the monthly breakdown must be per person: $150,000,000 / 10,000 / 12 = $1,250. The founder also confirmed that school remains separately allocated to enrolled households. This supersedes the previous interpretation of 10,000 as families. Housing, healthcare, utilities, school, community centers, parks, recycling, waste management, and compost/mulch operations remain included. Figures are planning inputs, not verified costs or current service prices.

## Budget boundary

| Annual allocation | Amount | Treatment |
| --- | ---: | --- |
| School | $10,000,000 | Retained illustrative allocation, including school overhead |
| Hospital | $11,000,000 | Retained illustrative allocation, within non-school upkeep |
| Remaining operations | $129,000,000 | Includes new recovery facilities, housing, utilities, parks, and other operations; individual budgets remain unitemized |
| Total | $150,000,000 | Each expense counted once |

The full-budget comparison is $15,000 per person annually, or $1,250 monthly. Non-school upkeep is $140 million, shared by 10,000 residents: $14,000 annually, or $1,166.67 monthly per person. Multiply the exact non-school rate by household members, then add school participation and optional support. The $1,250 average spreads school across all residents for comparison; actual household estimates allocate it to enrolled children instead. Recycling revenue, facility prices, and savings are not invented or deducted. This is annual upkeep, not construction financing or an established individual healthcare package.

## Allocation contract

Defaults: total budget B=$150m, school S=$10m, community residents N=10,000, enrolled students E=2,000 (illustrative), annual school support from others D=0, household members H=2, its enrolled children K=0, optional monthly school support G=0, and absent comparison price F. All money is USD.

- Community counts represent people. Each household member, including a child, receives one shared-upkeep share.
- Other upkeep = B - S. Monthly per-person requirement = (B - S) / (12 * N). Household other upkeep = H * (B - S) / (12 * N).
- Annual school support = D + 12 * G. D excludes this family's support. Applied support = min(S, support); excess remains separately unallocated.
- Remaining school requirement = max(0, S - support). Required monthly student rate = remaining / (12 * E) when E > 0. With E = 0, show an unavailable rate and the remaining need.
- Household total = H * other-upkeep rate + K * school rate + G. No enrolled children means no required school charge in this example; final eligibility remains open.
- The per-person comparison divides the exact family total by H. It is a display comparison, not a charging rule. Annual figures multiply exact monthly components by 12 before rounding.
- Optional price F produces annual school contributions of 12 * E * F. Add assumed support to show the gap or surplus against S. Its separate household total uses H * other-upkeep rate + K * F + G, without replacing the primary required rate.
- Annual contributions across N residents + school participant requirements + applied support reconcile to B. At zero enrollment, the remaining unfunded school requirement replaces participant charges. Surplus is additional and separate.

School and hospital inputs stay at their earlier illustrative values until revised. The resident count is an explicit input, not a family count. Community enrollment stays independent of household edits. General school support benefits all enrolled students; targeted assistance needs separate policy.

## Validation and precision

Decimal-to-cents parsing accepts nonnegative money with at most two decimal places. Counts must be whole numbers. Money fields are limited to $1 trillion and counts to 1 billion, with checked integer intermediates and rational rates. Only final display rounds half-up to cents.

Require N > 0, 1 <= H <= N, 0 <= S <= B, E <= N, K <= H, and K <= E. Require E - K <= N - H: students outside the household must fit within the residents outside it. When a household contains the entire community, its enrollment must match community enrollment.

Invalid input remains visible with a nearby error and no stale estimate. An invalid optional comparison leaves the primary result usable. Blank comparison differs from a zero-dollar test. Late field-change announcements during route changes ignore disposed calculator state.

## Reference examples

At 10,000 residents and 2,000 students:

| Family scenario | Monthly total | Annual total |
| --- | ---: | ---: |
| One member, no enrolled children, no support | $1,166.67 | $14,000.00 |
| Two members, no enrolled children, no support (default) | $2,333.33 | $28,000.00 |
| Five members, no enrolled children, no support | $5,833.33 | $70,000.00 |
| Five members, one enrolled child, no support | $6,250.00 | $75,000.00 |
| Five members, two enrolled children, no support | $6,666.67 | $80,000.00 |
| Four members, two enrolled children, $2m school support from others | $5,333.33 | $64,000.00 |
| Four members, two enrolled children, own $50 monthly school support | $5,549.95 | $66,599.40 |

The $300 school-price comparison still leaves a $2.8m annual gap without support, or $800,000 with $2m support. Annual totals never multiply already-rounded monthly display amounts.

## Interface, evidence, and remaining decisions

`#/costs` retains three views: How it works, Household calculator, and Funding & accountability, and links to `#/recovery`. Expanded household/community breakdowns persist through edits and invalid-input recovery. Reset restores input defaults. Values exist only in component memory. No enrollment, payment, pledge, coverage, vote, investment, browser storage, or URL-encoded household data is created. Donation and interest forms remain separate.

Thirteen tests target this resident-based contract, including 920 exact reconciliation scenarios. The calculator displays the full-budget per-person formula above household results and the shared-upkeep/school formulas beside each amount. The original handoff's companion JSON fixtures were unavailable. Historical verification records describe earlier assumptions and are not the current price model. See `VERIFICATION.md` for actual checks.

Open decisions include evidenced facility budgets and capacity; selected development and accountable operators; independent service choices; infrastructure obligations; school eligibility and schooling elsewhere; healthcare scope; participation periods and withdrawal; minimum funding commitments; hardship and surplus policies; governance membership, voting, and management accountability. A working demo does not establish operating readiness or economic feasibility.
