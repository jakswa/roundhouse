# Roundhouse

Another stab at a marta.io clone, this time
server-side-rendered and in rust with HTTP compression.

Features:
- three views: station list, station view, train view
- dark/light mode and other styling using tailwindcss
- polls every 10s for updates if you sit in a view (htmx/morphing)

![image](https://github.com/jakswa/roundhouse/assets/137793/f7725b7e-1cc0-477f-b020-3534fe48039c)

## Quick Start

0. install `bun` and `cargo`/rust
1. `cargo install ultraman cargo-watch`
2. `ultraman start -f Procfile.dev`
  - this kicks of `tailwindcss` watching your templates for style changes
  - kicks off cargo-watch rebuilding your app any time you edit files

(there is no live-reloading dev experience)

### Welcome to Loco :train:

This was scaffolded using Loco, which is a web and API framework running on Rust.

This is the **lightweight-service starter** which comes with no database or state dependencies.
