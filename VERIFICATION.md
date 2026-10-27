# Verification

Community costs and choice, 2026-10-27:

- Ten native pure-model tests pass (`rustc --edition 2024 --test src/cost_model.rs`). They cover the written reference examples A–I, own-household support accounting, zero enrollment and budgets, surplus, strict parsing and count compatibility, supported numeric limits, exact half-up rounding, inactive versus zero-price comparison, and 920 exact annual reconciliation scenarios. The referenced companion JSON files were unavailable; these tests derive from the written contract.
- Rust formatting, Git whitespace checks, and the production Trunk/WebAssembly build pass. No dependencies or external services were added.
- Chromium interaction checks confirm household totals of $2,166.67 (four residents/two students), $2,216.62 (same household plus $50 monthly support), and $2,000.00 (four/two with $2m support from others). The $300 test price reveals annual gaps of $2.8m without support and $800,000 with $2m support. Lower enrollment produces $833.33 per student.
- Zero enrollment displays an unavailable rate and the remaining $10m school requirement. Full support at zero enrollment retains the no-students explanation; excess support displays a separate $1m surplus in the tested scenario. Invalid counts, empty required fields, and extra decimal places remove the estimate without changing the user's input. An invalid alternative price leaves the primary calculation intact. Reset restores all nine fields and clears errors.
- Browser request instrumentation recorded zero network requests during calculator edits, validation, comparison, and reset. No payment or email was sent. The feature source contains no browser-storage writes or URL serialization of inputs.
- All three views remain reachable without horizontal document overflow at 1440×900, 1280×640, 768×1024, and 360×800. A 720×450 effective viewport was checked for 200% browser-zoom-equivalent reflow; 200% CSS zoom was separately exercised with editable assumptions and visible results. Native browser toolbar zoom was not independently audited.
- Desktop overview/calculator and mobile overview/results screenshots were reviewed. Native expandable sections, linked labels/help/errors, keyboard field order, reduced-motion state, and a polite edit-commit summary were checked. Screen-reader audio output was not independently audited.
- Homepage and main-navigation links reach `#/costs`, the active link exposes `aria-current="page"`, the document title updates, and browser back/forward works. Atlas, family community, roadmap, and interest screens still render. Desktop and mobile entrance checks show no hero/metrics overlap after adding the compact costs link.
- PayPal retains its supplied action, POST method, `_top` target, and button ID `EX59364B55S2A`. The interest form retains its FormSubmit recipient action. This release did not repeat a real payment or email-delivery test.
- No application console errors were observed. The pre-existing generated module-preload integrity warning remains informational.

The operating figures are founder-supplied planning assumptions. These checks establish calculator behavior and presentation, not economic feasibility, actual funding, enrollment, housing availability, healthcare coverage, or operating readiness.

PayPal support-button update, 2026-10-27:

- Release build and formatting checks pass. The original supplied action, POST method, hosted button ID `EX59364B55S2A`, and `_top` target are preserved in the custom-styled button.
- A browser-intercepted click verified the exact payload `hosted_button_id=EX59364B55S2A`. This verification did not initiate a real PayPal payment or inspect the receiving account.
- Entrance layouts reviewed at 1440×900, 1366×768, 390×844, and 360×800. No horizontal page overflow or overlap between the hero copy and metric strip was found.

Entrance and early-interest update, 2026-10-27:

- Release compilation passes with the signup and entrance components implemented in Leptos. The form has no custom email server or exposed secret.
- Browser validation blocks empty required fields, invalid email, and an unchecked consent box. Moving back from review retains the entered values. A script-shaped note remains text.
- A browser-intercepted native POST confirmed the exact FormSubmit recipient URL, all entered fields, consent acknowledgment, subject, blank honeypot, and absolute thank-you URL. That test was intercepted and sent no email.
- A separate real, clearly labeled setup request reached FormSubmit from the published site's origin and triggered activation. The inbox owner confirmed activation; a subsequent labeled delivery test returned HTTP 200 and `success: "true"`. Provider acceptance is verified. The destination inbox and native provider CAPTCHA/redirect flow were not independently verified end to end.
- Desktop and mobile entrance/signup layouts were inspected at 1440×900, 1366×768, and 390×844 without document horizontal overflow. On desktop, the form uses separate contact and review steps. Mobile panels stack.
- Pointer movement changes the scene's parallax parameters. The pause button and operating-system reduced-motion preference stop the image, orbit, particles, energy, and headline animations. The signup and ecosystem-entry links navigate correctly.
- A `pageshow` listener restores the submit button after returning from the provider, and is removed when the form component unmounts.

Family-life and connected-atlas release, 2026-10-27.

- The Leptos client compiles for `wasm32-unknown-unknown` with `trunk build --release --public-url ./`. Rust formatting and Git whitespace checks pass.
- Content validation confirms 24 unique domain IDs, 20 unique place IDs, valid cross-domain references, and six parseable SVG assets.
- All 20 place routes and 80 place-detail tabs opened and populated in Chromium. Place pagination, category filtering, and empty search results passed.
- All 24 domain routes and 120 domain-detail tabs opened and populated. Every domain exposes distinct software, service-operations, and physical-infrastructure readiness panels; these are explicitly unverified.
- All 11 atlas district buttons were exercised with ordinary pointer clicks. All 33 district/mode combinations produced labeled connections with no label-to-label overlap at 1366×768. The default atlas starts without connections; a Housing water-supply label successfully navigates to the Water domain. Hide lines clears the overlay before selecting another district.
- All nine school spaces and six park zones update their inspectors. Design-and-operation tabs contain the associated space details.
- All 15 moments across the three household journeys update the explanation and highlight one corresponding neighborhood place.
- All five proposed-deployment panels and the final independence stage passed. The animation toggle applies the reduced-motion state.
- Responsive checks covered 16 representative routes at both 1366×768 and 390×844: no document horizontal overflow; no document vertical scrolling on desktop. The atlas, neighborhood, school, park, and support-diagram default inspectors fit without internal scrolling at the desktop size. Longer detail chapters remain scrollable within their panels. Mobile layouts stack and their navigation strips scroll horizontally within the page.
- Screenshots of the entrance, family neighborhood, school, park, connections, atlas overlays, and mobile detail pages were reviewed. The original tiger artwork and dark blue visual direction are preserved.
- No application console errors were observed. Chromium retains the informational generated module-preload integrity warning; loading succeeds.
- The unchanged share implementation was verified in the preceding release to send the actual page URL to the Clipboard API successfully; readable OS clipboard content was unavailable to the automation environment.

The physical models, household journeys, and proposed deployment are conceptual. These checks verify the presentation software; they do not demonstrate operating readiness, a commissioned facility, compliance, or ecosystem independence.
