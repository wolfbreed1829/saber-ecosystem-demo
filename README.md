# Saber Tech — The Big One

A cinematic executive walkthrough of the Saber Tech ecosystem, implemented in **Rust / Leptos 0.8 client-side rendering** and compiled to WebAssembly with Trunk.

## Experience

- Blue tiger and Earth entrance.
- Interactive isometric atlas of nine infrastructure districts.
- Twenty-four descriptive operating-domain pages, each with overview, operating model, dependencies, and evidence tabs.
- Five physical production chains with individually selectable stages.
- Six architecture layers, four authority perspectives, and six evidence stages.
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

`src/domains.json` is the descriptive domain content; `src/pages.rs` contains the walkthrough components; `style.css` defines the visual system. `generate_atlas.py` reproducibly generates the original vector architectural illustration.

## Assets

- `public/tiger-earth.png`: generated for this project using the built-in image-generation tool. The prompt is recorded in [ARTWORK.md](ARTWORK.md).
- `public/atlas.svg`: original code-generated vector illustration.
- `public/fonts/space-grotesk.woff2`: locally hosted Space Grotesk; license in `public/fonts/OFL.txt`.

The executive demo is intended for GitHub Pages publication. That does not change the local-only decision for the separate blueprint repository.
