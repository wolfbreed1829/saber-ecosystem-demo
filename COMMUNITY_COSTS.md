# Community costs and choice - current model

The founder revised the planning budget to **$150 million per year for upkeep shared across 10,000 families**, adding a recycling plant, waste management, and a food-waste compost/mulch facility. This supersedes the earlier resident-based handoff. Housing, healthcare, utilities, school, community centers, parks, and other operations remain included. Figures are hypothetical planning inputs, not verified costs or current service prices.

## Budget boundary

| Annual allocation | Amount | Treatment |
| --- | ---: | --- |
| School | $10,000,000 | Retained illustrative allocation, including school overhead |
| Hospital | $11,000,000 | Retained illustrative allocation, within non-school upkeep |
| Remaining operations | $129,000,000 | Includes new recovery facilities, housing, utilities, parks, and other operations; individual budgets remain unitemized |
| Total | $150,000,000 | Each expense counted once |

The equal-share comparison is $15,000 per family annually, or $1,250 monthly, before the existing school-choice allocation. Non-school upkeep is $140 million, shared once by each family: $14,000 annually, or $1,166.67 monthly. The school contribution depends on enrollment and assumed support. Recycling revenue, facility prices, and savings are not invented or deducted. This is annual upkeep, not construction financing or an established individual healthcare package.

## Allocation contract

Defaults: total budget B=$150m, school S=$10m, participating families N=10,000, enrolled students E=2,000 (illustrative), annual school support from others D=0, members of the displayed family H=2, its enrolled children K=0, optional monthly school support G=0, and absent comparison price F. All money is USD.

- One displayed household is one family. Member count H does **not** multiply its shared-services charge.
- Other upkeep = B - S. Monthly per-family requirement = (B - S) / (12 * N).
- Annual school support = D + 12 * G. D excludes this family's support. Applied support = min(S, support); excess remains separately unallocated.
- Remaining school requirement = max(0, S - support). Required monthly student rate = remaining / (12 * E) when E > 0. With E = 0, show an unavailable rate and the remaining need.
- Family total = one other-upkeep share + K * school rate + G. No enrolled children means no required school charge in this example; final eligibility remains open.
- The per-person comparison divides the exact family total by H. It is a display comparison, not a charging rule. Annual figures multiply exact monthly components by 12 before rounding.
- Optional price F produces annual school contributions of 12 * E * F. Add assumed support to show the gap or surplus against S. Its separate family total uses one other-upkeep share + K * F + G, without replacing the primary required rate.
- Annual contributions across N families + school participant requirements + applied support reconcile to B. At zero enrollment, the remaining unfunded school requirement replaces participant charges. Surplus is additional and separate.

School and hospital inputs stay at their earlier illustrative values until revised. Total population is not inferred from family count, the displayed family size, or student count. Community enrollment stays independent of household edits. General school support benefits all enrolled students; targeted assistance needs separate policy.

## Validation and precision

Decimal-to-cents parsing accepts nonnegative money with at most two decimal places. Counts must be whole numbers. Money fields are limited to $1 trillion and counts to 1 billion, with checked integer intermediates and rational rates. Only final display rounds half-up to cents.

Require N > 0, H >= 1, 0 <= S <= B, K <= H, and K <= E. Students can outnumber families; H is not capped by N. With N = 1, require E = K because no other participating family could contain remaining students. No resident-population constraint is invented when population is unspecified.

Invalid input remains visible with a nearby error and no stale estimate. An invalid optional comparison leaves the primary result usable. Blank comparison differs from a zero-dollar test. Late field-change announcements during route changes ignore disposed calculator state.

## Reference examples

At 10,000 families and 2,000 students:

| Family scenario | Monthly total | Annual total |
| --- | ---: | ---: |
| Any valid member count, no enrolled children, no support | $1,166.67 | $14,000.00 |
| Four members, one enrolled child, no support | $1,583.33 | $19,000.00 |
| Four members, two enrolled children, no support | $2,000.00 | $24,000.00 |
| Four members, two enrolled children, $2m school support from others | $1,833.33 | $22,000.00 |
| Four members, two enrolled children, own $50 monthly school support | $2,049.95 | $24,599.40 |

The $300 school-price comparison still leaves a $2.8m annual gap without support, or $800,000 with $2m support. Annual totals never multiply already-rounded monthly display amounts.

## Interface, evidence, and remaining decisions

`#/costs` retains three views: How it works, Household calculator, and Funding & accountability, and links to `#/recovery`. Expanded household/community breakdowns persist through edits and invalid-input recovery. Reset restores input defaults. Values exist only in component memory. No enrollment, payment, pledge, coverage, vote, investment, browser storage, or URL-encoded household data is created. Donation and interest forms remain separate.

Thirteen tests target this family-based contract, including 2,600 exact reconciliation scenarios. The original handoff's companion JSON fixtures were unavailable. Historical verification records describe earlier assumptions and are not the current price model. See `VERIFICATION.md` for actual checks.

Open decisions include evidenced facility budgets and capacity; selected development and accountable operators; independent service choices; infrastructure obligations; school eligibility and schooling elsewhere; healthcare scope; participation periods and withdrawal; minimum funding commitments; hardship and surplus policies; governance membership, voting, and management accountability. A working demo does not establish operating readiness or economic feasibility.
