# Saber Tech — The Big One

A cinematic digital blueprint of the Saber Tech ecosystem and the everyday life it supports, implemented in **Rust / Leptos 0.8 client-side rendering** and compiled to WebAssembly with Trunk.

## Experience

- Blue tiger and Earth entrance.
- Interactive isometric atlas with eleven district entries, including Nexus education and civic participation. Select a district to reveal labeled, navigable connections across physical supply, digital coordination, and decision authority.
- A family neighborhood, a nine-space learning campus, and a six-zone discovery park, each with original vector illustrations and selectable places.
- Twenty everyday-place blueprints covering homes, school, childcare, playgrounds, library, sports, swimming, arts, teen spaces, elder support, family care, daily essentials, accessibility, transport, and more.
- Three five-step household journeys that explain the experience, record permissions, and decision authority at each moment.
- Five interactive dependency diagrams connecting everyday experiences to accountable operating domains.
- Twenty-four descriptive operating-domain pages, each with overview, operating model, dependencies, evidence required, and separate software / service / physical readiness tabs.
- Five physical production chains with individually selectable stages.
- Six architecture layers, four authority perspectives, and six evidence stages.
- A proposed first deployment: a complete housing-maintenance request workflow, with users, operator responsibilities, dependencies, and proposed acceptance criteria. It is a planning proposal, not an announced launch.
- Search, sector filters, pagination, hash-based deep links, copy-link sharing, reduced motion, keyboard-accessible controls, and responsive layouts.

The visual atlas is a conceptual arrangement, not a selected development site or engineering plan. The content describes proposed capabilities and preserves explicit evidence boundaries. The private blueprint repository and source PDFs are not included.

## Local development

Install Rust, the `wasm32-unknown-unknown` target, and Trunk. Then:

```text
rustup target add wasm32-unknown-unknown
trunk serve
```

If an inherited `NO_COLOR=1` makes Trunk reject its boolean setting, set `NO_COLOR=true` for the command.

## Production

```text
trunk build --release --public-url ./
```

The output is `dist/`. Hash navigation keeps every section directly shareable on GitHub Pages without a server-side router. The application has no backend or credentials and makes no application-data requests to external services.

The checked-in `docs/` directory is the verified release bundle used by GitHub Pages (main branch, `/docs`). Rebuild and refresh that directory when publishing an update. See [verification](VERIFICATION.md).

`src/domains.json` is the operating-domain content. `src/places.json` is the editable everyday-place content. `src/community_data.rs` defines campus and park spaces; `src/community.rs` implements the family views, journeys, and support diagrams. `src/atlas.rs` implements connection overlays; `src/readiness.rs` defines evidence status and the proposed first deployment. `style.css` preserves the established dark blue visual system.

Run `python generate_atlas.py`, `python generate_community.py`, and `python generate_atlas_extensions.py` to regenerate the original vector illustrations. These are conceptual arrangements, not dimensioned construction or playground equipment designs.

Twelve main routes are directly shareable through hash navigation, plus 24 domain and 20 place routes. Start with `#/community` for family life, `#/learning` for the campus, `#/play` for the park, or `#/day` for a guided household journey.

## Assets

- `public/tiger-earth.png`: generated for this project using the built-in image-generation tool. The prompt is recorded in [ARTWORK.md](ARTWORK.md).
- `public/atlas.svg`: original code-generated vector illustration.
- `public/atlas-extended.svg`, `public/neighborhood.svg`, `public/school-campus.svg`, and `public/playground.svg`: original code-generated vector concept models.
- `public/fonts/space-grotesk.woff2`: locally hosted Space Grotesk; license in `public/fonts/OFL.txt`.

The executive demo is intended for GitHub Pages publication. That does not change the local-only decision for the separate blueprint repository.

## Content and evidence boundaries

The documented strategy supplies the ecosystem's proposed domains and separation of operators, coordination, and authority. The family-space layouts, household scenarios, and first-deployment workflow are proposed elaborations for this digital blueprint. No traction, commissioned capacity, funding ask, launch date, or operating certification is invented. Domain software, service operations, and physical infrastructure remain **unverified** until supporting records are published; a working presentation proves only that the presentation runs.

Planning references used for the playground, early care, and access concepts: [CPSC Public Playground Safety Handbook](https://www.cpsc.gov/Playground-Handbook), [ChildCare.gov quality-care guidance](https://www.childcare.gov/consumer-education/find-and-choose-quality-child-care), and [U.S. Access Board ADA standards](https://www.access-board.gov/ada/). These inform design questions; they do not establish compliance for a selected facility.
