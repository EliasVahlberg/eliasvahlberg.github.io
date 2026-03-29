# Design Notes — eliasvahlberg.github.io

## Theme

Inspired by the Garuda Linux Dr460nized / Sweet-Dark terminal theme. The goal is a terminal-aesthetic developer portfolio that feels authentic — not a costume, but a reflection of the actual work (TUI software, terminal tools, Rust).

## Color Palette

Based on the official Garuda Dr460nized Konsole/Alacritty configuration:

| Role | Hex | Usage |
|---|---|---|
| Background | `#0a1124` | Main bg, terminal bg |
| Sidebar bg | `#080e1e` | Slightly darker |
| Card bg | `#0d1530` | Code blocks |
| Border | `#1a2540` | Dividers |
| Text | `#eec49a` | Body text (warm peach/off-white) |
| Dim text | `#a6896b` | Muted text |
| Secondary | `#d4a87a` | Nav links, taglines, meta — readable but not primary |
| Accent | `#F6A73B` | Links, icons, copy buttons, cursor (amber) |
| Green slot | `#FAD32F` | Section labels, crate names (yellow — Garuda cyan slot) |
| Blue slot | `#F35645` | Prompt path, errors (red-orange — Garuda blue slot) |

Icon gradient: `#FAD32F` → `#F6A73B` → `#F35645` (yellow → amber → red-orange)

## Layout

Three-column sticky layout:
- **Sidebar** (280px, sticky): name, role, nav, about blurb, published crates
- **Content** (780px, fixed width): projects, publications, footer
- **Terminal panel** (flex: 1, fills remaining space): interactive terminal

Terminal panel hides below 1100px viewport width. Sidebar collapses to top bar below 768px.

## Terminal Panel

Frosted glass effect:
- Semi-transparent dark navy gradient background
- `backdrop-filter: blur(8px)` — blurs content behind it
- Left border: `rgba(250, 211, 47, 0.35)` amber glow
- Box shadow: warm amber outer glow

Interactive commands: `ls`, `cat <project>`, `whoami`, `crates`, `crate-stats`, `git log`, `terrain-forge <algo>`, `clear`, `help`

Live data: `crate-stats` hits crates.io API, `git log` hits GitHub API.

WASM demo: `terrain-forge <algo>` runs actual terrain-forge Rust code compiled to WASM in the browser. Algorithms: bsp, cellular, drunkard, maze, rooms, dla, voronoi.

Tab autocomplete with inline ghost text (fish shell style).

## SVG Icons

Each project has a small 22×22px icon next to its heading. All use a shared gradient (`url(#icon-grad)`) for the stroke — yellow → amber → red-orange. Drop shadow filter adds a subtle amber glow. Icons are stroke-based (no fill) for a clean line-art look.

- saltglass-steppe: hexagon with cross lines (crystal/glass shard)
- terrain-forge: mountain silhouette with baseline
- fishy: fish with tail fin and eye dot
- ogun + oku: triangle node graph
- chess engine: knight piece silhouette

## Typography

- Monospace: JetBrains Mono (loaded from Google Fonts), fallback chain: Fira Code → Cascadia Code → Consolas
- Sans: Inter, system-ui
- Section labels, nav, code blocks, terminal: monospace
- Body text: sans

## Content Philosophy

- Personal and authentic — projects described with enthusiasm for the work, not as a CV
- "Fun project backed by actual science" is the signature: each project has a research angle (DS theory, Glass Seam Bridging algorithm, sequential logit dynamics, chess search theory)
- Academic publications and technical write-ups separated clearly — no stolen-valor ambiguity
- Job title stated plainly without over-claiming degree status

## Stack

Plain HTML + CSS + minimal vanilla JS. No frameworks, no npm, no build step. One WASM module (`pkg/`) compiled from Rust via wasm-pack. Deployed to GitHub Pages from `main` branch root.
